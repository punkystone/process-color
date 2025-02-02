use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use tauri::{AppHandle, State};
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_opener::OpenerExt;

use crate::{
    log, mqtt::MqttClient, mqtt_settings::MqttSettings, process_entry::ProcessEntry,
    storage::Storage,
};

#[tauri::command]
pub fn add_process_entry(
    state: State<Arc<Mutex<Vec<ProcessEntry>>>>,
    storage: State<Arc<Mutex<Storage>>>,
) {
    let process_entrys = state.lock();
    if process_entrys.is_err() {
        log("failed to lock process entrys");
        return;
    }
    let mut process_entrys = process_entrys.unwrap();
    let storage = storage.lock();
    if storage.is_err() {
        log("failed to lock storage");
        return;
    }
    let storage = storage.unwrap();

    process_entrys.push(ProcessEntry {
        is_running: false,
        name: String::new(),
        topic: String::new(),
        value: String::new(),
        off_value: String::new(),
    });
    storage.save_process_entrys(&process_entrys);
}

#[tauri::command]
pub fn delete_process_entry(
    index: usize,
    state: State<Arc<Mutex<Vec<ProcessEntry>>>>,
    storage: State<Arc<Mutex<Storage>>>,
) {
    let process_entrys = state.lock();
    if process_entrys.is_err() {
        log("failed to lock process entrys");
        return;
    }
    let mut process_entrys = process_entrys.unwrap();
    let storage = storage.lock();
    if storage.is_err() {
        log("failed to lock storage");
        return;
    }
    let storage = storage.unwrap();

    process_entrys.remove(index);
    storage.save_process_entrys(&process_entrys);
}

#[tauri::command]
pub fn mqtt_connect(state: State<Arc<Mutex<MqttClient>>>) {
    let mqtt_client = state.lock();
    if mqtt_client.is_err() {
        log("failed to lock mqtt client");
        return;
    }
    let mut mqtt_client = mqtt_client.unwrap();
    mqtt_client.connect();
}
#[tauri::command]
pub fn get_mqtt_connection(mqtt_client: State<Arc<Mutex<MqttClient>>>) -> Option<MqttSettings> {
    let mqtt_client = mqtt_client.lock();
    if mqtt_client.is_err() {
        log("failed to lock mqtt client");
        return None;
    }
    let mqtt_client = mqtt_client.unwrap();
    return mqtt_client.settings.clone();
}

#[tauri::command]
pub fn save_mqtt_connection(
    mqtt_client: State<Arc<Mutex<MqttClient>>>,
    storage: State<Arc<Mutex<Storage>>>,
    ip: String,
    port: u16,
) {
    let mqtt_client = mqtt_client.lock();
    if mqtt_client.is_err() {
        log("failed to lock mqtt client");
        return;
    }
    let mut mqtt_client = mqtt_client.unwrap();

    let storage = storage.lock();
    if storage.is_err() {
        log("failed to lock storage");
        return;
    }
    let storage = storage.unwrap();

    let settings = MqttSettings { ip, port };
    storage.save_mqtt_settings(&settings);
    mqtt_client.settings = Some(settings);
    mqtt_client.connect();
}

#[tauri::command]
pub fn open_config(app: AppHandle, storage: State<Arc<Mutex<Storage>>>) {
    let storage = storage.lock();
    if storage.is_err() {
        log("failed to lock storage");
        return;
    }
    let storage = storage.unwrap();

    let path = &storage.path;
    if path.is_none() {
        return;
    }
    let result = app
        .opener()
        .open_path(path.as_ref().unwrap().display().to_string(), None::<&str>);
    if result.is_err() {
        log("failed to open config directory");
        return;
    }
}

#[tauri::command]
pub fn get_process_entrys(state: State<Arc<Mutex<Vec<ProcessEntry>>>>) -> Vec<ProcessEntry> {
    let entries = state.lock();
    if entries.is_err() {
        log("failed to lock process entries");
        return vec![];
    }
    let entries = entries.unwrap();
    entries.clone()
}

#[tauri::command]
pub fn get_processes(state: State<Arc<Mutex<HashSet<String>>>>) -> Vec<String> {
    let processes = state.lock();
    if processes.is_err() {
        log("failed to lock processes");
        return vec![];
    }
    let processes = processes.unwrap();
    let mut processes = processes.clone().into_iter().collect::<Vec<String>>();
    processes.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    processes
}

#[tauri::command]
pub fn update_process_entry(
    state: State<Arc<Mutex<Vec<ProcessEntry>>>>,
    storage: State<Arc<Mutex<Storage>>>,
    index: usize,
    name: String,
    topic: String,
    value: String,
    off_value: String,
) {
    let process_entrys = state.lock();
    if process_entrys.is_err() {
        log("failed to lock process entrys");
        return;
    }
    let mut process_entrys = process_entrys.unwrap();
    let storage = storage.lock();
    if storage.is_err() {
        log("failed to lock storage");
        return;
    }
    let storage = storage.unwrap();

    for (i, entry) in process_entrys.iter_mut().enumerate() {
        if i == index {
            entry.name = name.clone();
            entry.topic = topic.clone();
            entry.value = value.clone();
            entry.off_value = off_value.clone();
            break;
        }
    }
    storage.save_process_entrys(&process_entrys);
}

#[tauri::command]
pub fn set_autostart(app: AppHandle, enabled: bool) {
    let autostart_manager = app.autolaunch();
    if enabled {
        let result = autostart_manager.enable();
        if result.is_err() {
            log("failed to enable autostart");
        }
    } else {
        let result = autostart_manager.disable();
        if result.is_err() {
            log("failed to disable autostart");
        }
    }
}

#[tauri::command]
pub fn get_autostart(app: AppHandle) -> bool {
    let autostart_manager = app.autolaunch();
    let result = autostart_manager.is_enabled();
    if result.is_err() {
        log("failed to check autostart status");
        return false;
    }
    result.unwrap()
}
