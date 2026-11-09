use crate::structs::LocationId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LocationChecksPacket {
    pub locations: Vec<LocationId>,
}
