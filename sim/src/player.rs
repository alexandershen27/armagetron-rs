use crate::cycle::CycleId;

pub type PlayerUid = u32;

pub struct Player {
    pub uid: PlayerUid, 
    pub cycle_id: Option<CycleId>,
}