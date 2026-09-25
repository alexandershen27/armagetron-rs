use render::{Camera, CameraUniform, Vertex, wall_pipeline};
use sim::game::{Cardinal, Game, PlayerUid};

use wgpu::util::DeviceExt;
use winit::window::Window;

use std::sync::Arc;

const MAX_WALLS: u64 = 128;

pub struct State {
    uid: PlayerUid,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    render_pipelines: [wgpu::RenderPipeline; 2],
    window: Arc<Window>,
    vertex_buffer: wgpu::Buffer,
    mesh_index_buffer: wgpu::Buffer,
    line_index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl State {
    pub async fn new(uid: PlayerUid, window: Arc<Window>) -> anyhow::Result<Self> {
        // Initialize instance, device, and queue
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
                apply_limit_buckets: true,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        // Initialize surface and config
        let size = window.inner_size();
        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            color_space: wgpu::SurfaceColorSpace::Auto,
        };

        let camera = Camera::new(
            (0.0, 60.0, 60.0).into(),
            (0.0, 0.0, 0.0).into(),
            glam::Vec3::Z,
            config.width as f32 / config.height as f32,
            45.0_f32.to_radians(),
            0.1,
            100.0,
        );

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let render_pipelines: [RenderPipelines; 2] = [
            mesh_pipeline(&device, config.format, &camera_bind_group_layout),
            line_pipeline(&device, config.format, &camera_bind_group_layout)
        ];

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: MAX_WALLS * 4 * std::mem::size_of::<Vertex>() as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mesh_index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Index Buffer"),
            size: MAX_WALLS * 6 * std::mem::size_of::<u32>() as u64,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let line_index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Index Buffer"),
            size: MAX_WALLS * 2 * std::mem::size_of::<u32>() as u64,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let num_indices = 0;

        Ok(Self {
            uid,
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            render_pipelines,
            window,
            vertex_buffer,
            mesh_index_buffer,
            line_index_buffer,
            num_indices,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    pub fn update(&mut self, game: Game) {
        // Camera
        let DISTANCE = 60.0;
        let HEIGHT = 60.0;
        if let Some(cycle) = game.grid().get_cycle_by_id(&self.uid) {
            let cycle_pos = glam::Vec3::new(cycle.position().x, cycle.position().y, 0.0);
            let dir = match cycle.facing() {
                Cardinal::North => glam::Vec3::new(0.0, 1.0, 0.0),
                Cardinal::South => glam::Vec3::new(0.0, -1.0, 0.0),
                Cardinal::East => glam::Vec3::new(1.0, 0.0, 0.0),
                Cardinal::West => glam::Vec3::new(-1.0, 0.0, 0.0),
            };

            let eye = cycle_pos - dir * DISTANCE + glam::Vec3::Z * HEIGHT;
            let center = cycle_pos;
            self.camera.eye(eye);
            self.camera.center(center);
            self.camera_uniform.update_view_proj(&self.camera);
            self.queue.write_buffer(
                &self.camera_buffer,
                0,
                bytemuck::cast_slice(&[self.camera_uniform]),
            );
        };

        // Draw walls
        let mut vertices: Vec<Vertex> = vec![];
        let mut indices: Vec<u32> = vec![];
        for (i, (_uid, wall)) in game
            .grid()
            .cycles()
            .into_iter()
            .flat_map(|(uid, cycle)| cycle.walls().iter().map(move |w| (uid, w)))
            .enumerate()
        {
            let z = 3.0;

            let sx = wall.start_position().x;
            let sy = wall.start_position().y;

            let ex = wall.end_position().x;
            let ey = wall.end_position().y;

            // Push to vertices
            let color = [1.0, 0.0, 0.0];

            // A (top start)
            vertices.push(Vertex {
                position: [sx, sy, z],
                color: color,
            });

            // B (top end)
            vertices.push(Vertex {
                position: [ex, ey, z],
                color: color,
            });

            // C (bottom end)
            vertices.push(Vertex {
                position: [ex, ey, 0.0],
                color: color,
            });

            // D (bottom start)
            vertices.push(Vertex {
                position: [sx, sy, 0.0],
                color: color,
            });

            let i = i as u32 * 4;
            indices.extend(vec![i, i + 1, i + 2, i, i + 2, i + 3])
        }

        self.queue
            .write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertices));

        self.queue
            .write_buffer(&self.mesh_index_buffer, 0, bytemuck::cast_slice(&indices));

        self.queue
            .write_buffer(&self.line_index_buffer, 0, bytemuck::cast_slice(&indices));
        q
        self.num_indices = indices.len() as u32;
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        if !self.is_surface_configured {
            return Ok(());
        }

        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => return Ok(()),
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Lost => anyhow::bail!("Lost device"),
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Renderer encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            render_pass.set_pipeline(&self.render_pipelines[0]);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.mesh_index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1)

            render_pass.set_pipeline(&self.render_pipelines[1]);
            render_pass.set_index_buffer()
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        self.queue.present(output);

        Ok(())
    }

    pub fn request_redraw(&mut self) {
        self.window.request_redraw();
    }
}
