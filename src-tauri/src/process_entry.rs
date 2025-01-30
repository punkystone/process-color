use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcessEntry {
    pub is_running: bool,
    pub name: String,
    pub topic: String,
    pub value: String,
    pub off_value: String,
}
