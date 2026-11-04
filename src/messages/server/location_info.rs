use serde::{Deserialize, Serialize};
use crate::messages::NetworkItem;

#[derive(Deserialize, Serialize, Clone)]
pub struct LocationInfoPacket {
    pub(crate) locations: Vec<NetworkItem>
}