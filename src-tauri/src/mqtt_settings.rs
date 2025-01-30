use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MqttSettings {
    pub ip: String,
    pub port: u16,
}
