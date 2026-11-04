pub mod connected;
pub mod connection_refused;
pub mod datapackage;
pub mod invalid;
pub mod received_items;
pub mod roominfo;
pub mod location_info;

use crate::messages::server::connected::ConnectedPacket;
use crate::messages::server::connection_refused::ConnectionRefusedPacket;
use crate::messages::server::datapackage::DataPackagePacket;
use crate::messages::server::invalid::InvalidPacketPacket;
use crate::messages::server::received_items::ReceivedItemsPacket;
use crate::messages::server::roominfo::RoomInfoPacket;
use serde::{Deserialize, Serialize};
use crate::messages::server::location_info::LocationInfoPacket;

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "cmd")]
pub enum ServerPacket {
    RoomInfo(RoomInfoPacket),
    DataPackage(DataPackagePacket),
    InvalidPacket(InvalidPacketPacket),
    Connected(ConnectedPacket),
    ConnectionRefused(ConnectionRefusedPacket),
    ReceivedItems(ReceivedItemsPacket),
    LocationInfo(LocationInfoPacket)
}
