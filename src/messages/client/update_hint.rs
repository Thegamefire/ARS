use crate::messages::client::create_hints::HintStatus;
use crate::structs::{LocationId, SlotId};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UpdateHintPacket {
    pub player: SlotId,
    pub location: LocationId,
    pub status: HintStatus,
}
