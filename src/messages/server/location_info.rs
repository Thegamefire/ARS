use crate::messages::NetworkItem;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct LocationInfoPacket {
    pub(crate) locations: Vec<NetworkItem>,
}
