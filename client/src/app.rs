use crate::state::State;

use sim::game::{Command, Direction, Game, Input, PlayerUid};

use std::io::{BufReader, prelude::*, stdin};
use std::net::TcpStream;
use std::sync::{Arc, mpsc};
use std::thread;

use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

pub struct App {
    uid: PlayerUid,
    state: Option<State>,
    reader: mpsc::Receiver<Game>,
    writer: TcpStream,
}

impl App {
    fn new(uid: PlayerUid, reader: mpsc::Receiver<Game>, writer: TcpStream) -> Self {
        App {
            uid,
            state: None,
            reader,
            writer,
        }
    }

    fn frame(&mut self, event_loop: &ActiveEventLoop) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        while let Ok(latest) = self.reader.try_recv() {
            state.update(latest);
        }

        match state.render() {
            Ok(_) => {}
            Err(e) => {
                log::error!("{e}");
                event_loop.exit();
            }
        }
    }
    fn handle_key(&mut self, _event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        let command = match (code, is_pressed) {
            (KeyCode::ArrowLeft, true) => Command::Direction(Direction::Left),
            (KeyCode::ArrowRight, true) => Command::Direction(Direction::Right),
            _ => return,
        };

        let input = Input {
            uid: self.uid as u32,
            command,
        };

        let _ = net::send(&mut self.writer, &input);
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        self.state = Some(pollster::block_on(State::new(self.uid, window)).unwrap());
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        state.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => self.frame(event_loop),
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => self.handle_key(event_loop, code, key_state.is_pressed()),
            _ => {}
        }
    }
}

pub fn run() -> anyhow::Result<()> {
    println!("Enter UID (one byte char): ");

    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Invalid input.");
    let uid = buf.as_bytes()[0];

    let mut writer =
        TcpStream::connect("127.0.0.1:8000").expect("Failed connect, is the server running?");

    // Send UID byte
    writer.write_all(&[uid]).unwrap();

    // Spawn stream
    let reader = writer.try_clone().unwrap();
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut reader = BufReader::new(&reader);

        loop {
            match net::recv::<Game>(&mut reader).expect("Read failed") {
                Some(snap) => {
                    let _ = tx.send(snap);
                }
                None => {
                    println!("Server disconnected.");
                    return;
                }
            }
        }
    });

    let event_loop = EventLoop::new()?;
    let mut app = App::new(uid as u32, rx, writer);
    event_loop.run_app(&mut app)?;

    Ok(())
}
