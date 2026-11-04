use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ConnectionRefusedPacket {
    pub errors: Vec<ConnectionRefusedReason>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ConnectionRefusedReason {
    InvalidSlot,
    InvalidGame,
    IncompatibleVersion,
    InvalidPassword,
    InvalidItemsHandling,
}
