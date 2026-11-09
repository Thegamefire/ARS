pub mod connect;
pub mod connect_update;
mod get_datapackage;
mod location_checks;
pub mod location_scouts;
pub mod create_hints;

use crate::messages::client::connect::ConnectPacket;
use crate::messages::client::connect_update::ConnectUpdatePacket;
use crate::messages::client::get_datapackage::GetDataPackagePacket;
use serde::{Deserialize, Serialize};
use crate::messages::client::create_hints::CreateHintsPacket;
use crate::messages::client::location_checks::LocationChecksPacket;
use crate::messages::client::location_scouts::LocationScoutsPacket;

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd")]
pub enum ClientPacket {
    GetDataPackage(GetDataPackagePacket),
    Connect(ConnectPacket),
    ConnectUpdate(ConnectUpdatePacket),
    Sync,
    LocationChecks(LocationChecksPacket),
    LocationScouts(LocationScoutsPacket),
    CreateHints(CreateHintsPacket)
}
