use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct InvalidPacketPacket {
    #[serde(rename = "type")]
    pub problem_type: PacketProblemType,
    pub original_cmd: Option<String>,
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum PacketProblemType {
    Cmd,
    Arguments,
}
