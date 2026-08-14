use crate::game::grid::CycleId;

pub type PlayerUid = u8;

#[derive(PartialEq)]
pub enum PlayerStatus {
    Active,
    Spectating,
    Inactive
}

pub struct Player {
    uid: PlayerUid, 
    status: PlayerStatus,
    cycle_id: Option<CycleId>,
}

impl Player {
    pub fn new(uid: PlayerUid) -> Self {
        Player {
            uid,
            status: PlayerStatus::Active,
            cycle_id: None
        }
    }
    
    pub fn get_uid(&self) -> PlayerUid { self.uid }
    pub fn is_active(&self) -> bool { self.status == PlayerStatus::Active }
    pub fn get_cycle_id(&self) -> Option<CycleId> { self.cycle_id }
    pub fn set_cycle_id(&mut self, id: CycleId) { self.cycle_id = Some(id) }
}