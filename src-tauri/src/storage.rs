use std::{
    fs::{create_dir_all, read, write},
    path::PathBuf,
};

use crate::{log, mqtt_settings::MqttSettings, process_entry::ProcessEntry};

pub struct Storage {
    pub path: Option<PathBuf>,
}
impl Storage {
    const PROCESS_ENTRYS_PATH: &'static str = "process_entrys.dat";
    const MQTT_SETTINGS_PATH: &'static str = "mqtt_settings.dat";
    pub fn new() -> Self {
        Self { path: None }
    }

    pub fn set_path(&mut self, path: String) {
        let path = PathBuf::from(path);

        if !path.exists() {
            let result = create_dir_all(&path);
            if result.is_err() {
                log("failed to create directory for storage");
            }
        }
        self.path = Some(path);
    }

    pub fn save_process_entrys(&self, process_entrys: &[ProcessEntry]) {
        let serialized = bincode::serialize(process_entrys);
        if serialized.is_err() {
            log("failed to serialize process entrys");
            return;
        }
        let serialized = serialized.unwrap();
        if self.path.is_none() {
            return;
        }
        let path = self.path.as_ref().unwrap();
        let result = write(path.join(Self::PROCESS_ENTRYS_PATH), serialized);
        if result.is_err() {
            log("failed to write process entrys");
        }
    }

    pub fn get_saved_process_entrys(&self) -> Vec<ProcessEntry> {
        if self.path.is_none() {
            return vec![];
        }
        let path = self.path.as_ref().unwrap();
        if !path.join(Self::PROCESS_ENTRYS_PATH).exists() {
            return vec![];
        }
        let data = read(path.join(Self::PROCESS_ENTRYS_PATH));
        if data.is_err() {
            log("failed to read process entrys");
            return vec![];
        }
        let data = data.unwrap();

        let deserialized = bincode::deserialize(&data);
        if deserialized.is_err() {
            log("failed to deserialize process entrys");
            return vec![];
        }
        deserialized.unwrap()
    }

    pub fn get_mqtt_settings(&self) -> MqttSettings {
        let default = MqttSettings {
            ip: "localhost".to_string(),
            port: 1883,
        };
        if self.path.is_none() {
            return default;
        }
        let path = self.path.as_ref().unwrap();
        if !path.join(Self::MQTT_SETTINGS_PATH).exists() {
            let serialized = bincode::serialize(&default);
            if serialized.is_err() {
                log("failed to serialize mqtt settings");
                return default;
            }
            let serialized = serialized.unwrap();

            let response = write(path.join(Self::MQTT_SETTINGS_PATH), serialized);
            if response.is_err() {
                log("failed to write mqtt settings");
            }
            return default;
        }
        let data = read(path.join(Self::MQTT_SETTINGS_PATH));
        if data.is_err() {
            log("failed to read mqtt settings");
            return default;
        }
        let data = data.unwrap();
        let deserialized = bincode::deserialize(&data);
        if deserialized.is_err() {
            log("failed to deserialize mqtt settings");
            return default;
        }
        deserialized.unwrap()
    }

    pub fn save_mqtt_settings(&self, settings: &MqttSettings) {
        if self.path.is_none() {
            return;
        }
        let path = self.path.as_ref().unwrap();
        let serialized = bincode::serialize(settings);
        if serialized.is_err() {
            log("failed to serialize mqtt settings");
            return;
        }
        let serialized = serialized.unwrap();
        let result = write(path.join(Self::MQTT_SETTINGS_PATH), serialized);
        if result.is_err() {
            log("failed to write mqtt settings");
        }
    }
}
