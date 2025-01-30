use std::{
    fs::{create_dir_all, read, write},
    path::PathBuf,
};

use crate::{mqtt_settings::MqttSettings, process_entry::ProcessEntry};

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
            create_dir_all(&path).unwrap();
        }
        self.path = Some(path);
    }

    pub fn save_process_entrys(&self, process_entrys: &[ProcessEntry]) {
        if self.path.is_none() {
            return;
        }
        let serialized = bincode::serialize(process_entrys).unwrap();
        let path = self.path.as_ref().unwrap();
        let _ = write(path.join(Self::PROCESS_ENTRYS_PATH), serialized);
    }

    pub fn get_saved_process_entrys(&self) -> Vec<ProcessEntry> {
        if self.path.is_none() {
            return vec![];
        }
        let path = self.path.as_ref().unwrap();
        if !path.join(Self::PROCESS_ENTRYS_PATH).exists() {
            return vec![];
        }
        let data = read(path.join(Self::PROCESS_ENTRYS_PATH)).unwrap();
        bincode::deserialize(&data).unwrap()
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
            let serialized = bincode::serialize(&default).unwrap();
            let _ = write(path.join(Self::MQTT_SETTINGS_PATH), serialized);
            return default;
        }
        let data = read(path.join(Self::MQTT_SETTINGS_PATH)).unwrap();
        bincode::deserialize(&data).unwrap()
    }

    pub fn save_mqtt_settings(&self, settings: &MqttSettings) {
        if self.path.is_none() {
            return;
        }
        let path = self.path.as_ref().unwrap();
        let serialized = bincode::serialize(settings).unwrap();
        let _ = write(path.join(Self::MQTT_SETTINGS_PATH), serialized);
    }
}
