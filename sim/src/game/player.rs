pub type PlayerUid = u32;

#[derive(PartialEq)]
pub enum PlayerStatus {
    Active,
    // Spectating,
    // Inactive,
}

pub struct Player {
    uid: PlayerUid,
    status: PlayerStatus,
}

impl Player {
    pub fn new(uid: PlayerUid) -> Self {
        Player {
            uid,
            status: PlayerStatus::Active,
        }
    }

    pub fn uid(&self) -> PlayerUid {
        self.uid
    }
    pub fn is_active(&self) -> bool {
        self.status == PlayerStatus::Active
    }
}
