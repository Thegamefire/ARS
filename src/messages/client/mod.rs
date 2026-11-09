pub mod connect;
pub mod connect_update;
pub mod create_hints;
mod get_datapackage;
mod location_checks;
pub mod location_scouts;
pub mod update_hint;

use crate::messages::client::connect::ConnectPacket;
use crate::messages::client::connect_update::ConnectUpdatePacket;
use crate::messages::client::create_hints::CreateHintsPacket;
use crate::messages::client::get_datapackage::GetDataPackagePacket;
use crate::messages::client::location_checks::LocationChecksPacket;
use crate::messages::client::location_scouts::LocationScoutsPacket;
use crate::messages::client::update_hint::UpdateHintPacket;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd")]
pub enum ClientPacket {
    GetDataPackage(GetDataPackagePacket),
    Connect(ConnectPacket),
    ConnectUpdate(ConnectUpdatePacket),
    Sync,
    LocationChecks(LocationChecksPacket),
    LocationScouts(LocationScoutsPacket),
    CreateHints(CreateHintsPacket),
    UpdateHint(UpdateHintPacket),
}
