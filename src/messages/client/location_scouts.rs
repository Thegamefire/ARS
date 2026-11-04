use serde::{Deserialize, Serialize, Serializer};
use crate::structs::LocationId;


#[derive(Serialize, Deserialize)]
pub struct LocationScoutsPacket {
    pub locations: Vec<LocationId>,
    pub create_as_hint: CreateAsHint
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(try_from = "i32")]
pub enum CreateAsHint {
    Hidden,
    BroadcastNew,
    BroadcastAll,
}

impl TryFrom<i32> for CreateAsHint {
    type Error = std::convert::Infallible;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Hidden,
            2 => Self::BroadcastNew,
            _ => Self::BroadcastAll,
        })
    }
}

impl Serialize for CreateAsHint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = match self {
            Self::Hidden => 0,
            Self::BroadcastNew => 2,
            Self::BroadcastAll => 1,
        };

        serializer.serialize_i32(value)
    }
}