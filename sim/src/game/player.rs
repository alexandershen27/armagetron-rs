use serde::{Deserialize, Serialize};

pub type PlayerUid = u32;

#[derive(PartialEq, Serialize, Deserialize)]
pub enum PlayerStatus {
    Active,
    // Spectating,
    Inactive,
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub uid: PlayerUid,
    pub status: PlayerStatus,
}

impl Player {
    pub fn new(uid: PlayerUid) -> Self {
        Player {
            uid,
            status: PlayerStatus::Active,
        }
    }
}
