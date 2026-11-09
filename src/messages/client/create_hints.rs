use crate::structs::{LocationId, SlotId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateHintsPacket {
    pub locations: Vec<LocationId>,
    pub player: Option<SlotId>,
    pub status: HintStatus,
}

#[derive(Copy, Clone, Serialize, Deserialize, Default)]
pub enum HintStatus {
    #[default]
    #[serde(rename = "HINT_UNSPECIFIED")]
    Unspecified, // The receiving player has not specified any status
    #[serde(rename = "HINT_NO_PRIORITY")]
    NoPriority, // The receiving player has specified that the item is unneeded
    #[serde(rename = "HINT_AVOID")]
    Avoid, // The receiving player has specified that the item is detrimental
    #[serde(rename = "HINT_PRIORITY")]
    Priority, // The receiving player has specified that the item is needed
    #[serde(rename = "HINT_FOUND")]
    Found, // The location has been collected. Status cannot be changed once found.
}
