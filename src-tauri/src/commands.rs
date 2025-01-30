use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use tauri::{AppHandle, State};
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_opener::OpenerExt;

use crate::{
    mqtt::MqttClient, mqtt_settings::MqttSettings, process_entry::ProcessEntry, storage::Storage,
};

#[tauri::command]
pub fn add_process_entry(
    state: State<Arc<Mutex<Vec<ProcessEntry>>>>,
    storage: State<Arc<Mutex<Storage>>>,
) {
    let mut process_entrys = state.lock().unwrap();
    process_entrys.push(ProcessEntry {
        is_running: false,
        name: String::new(),
        topic: String::new(),
        value: String::new(),
        off_value: String::new(),
    });
    storage.lock().unwrap().save_process_entrys(&process_entrys);
}

#[tauri::command]
pub fn delete_process_entry(
    index: usize,
    state: State<Arc<Mutex<Vec<ProcessEntry>>>>,
    storage: State<Arc<Mutex<Storage>>>,
) {
    let mut process_entrys = state.lock().unwrap();
    process_entrys.remove(index);
    storage.lock().unwrap().save_process_entrys(&process_entrys);
}

#[tauri::command]
pub fn mqtt_connect(state: State<Arc<Mutex<MqttClient>>>) {
    state.lock().unwrap().connect();
}
#[tauri::command]
pub fn get_mqtt_connection(mqtt_client: State<Arc<Mutex<MqttClient>>>) -> Option<MqttSettings> {
    return mqtt_client.lock().unwrap().settings.clone();
}

#[tauri::command]
pub fn save_mqtt_connection(
    mqtt_client: State<Arc<Mutex<MqttClient>>>,
    storage: State<Arc<Mutex<Storage>>>,
    ip: String,
    port: u16,
) {
    let settings = MqttSettings { ip, port };
    let mut mqtt_client = mqtt_client.lock().unwrap();
    storage.lock().unwrap().save_mqtt_settings(&settings);
    mqtt_client.settings = Some(settings);
    mqtt_client.connect();
}

#[tauri::command]
pub fn open_config(app: AppHandle, storage: State<Arc<Mutex<Storage>>>) {
    let path = &storage.lock().unwrap().path;
    if path.is_none() {
        return;
    }
    let _ = app
        .opener()
        .open_path(path.as_ref().unwrap().display().to_string(), None::<&str>);
}

#[tauri::command]
pub fn get_process_entrys(state: State<Arc<Mutex<Vec<ProcessEntry>>>>) -> Vec<ProcessEntry> {
    state.lock().unwrap().clone()
}

#[tauri::command]
pub fn get_processes(state: State<Arc<Mutex<HashSet<String>>>>) -> Vec<String> {
    let processes = state.lock().unwrap();
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
    let mut process_entrys = state.lock().unwrap();
    for (i, entry) in process_entrys.iter_mut().enumerate() {
        if i == index {
            entry.name = name.clone();
            entry.topic = topic.clone();
            entry.value = value.clone();
            entry.off_value = off_value.clone();
            break;
        }
    }
    storage.lock().unwrap().save_process_entrys(&process_entrys);
}

#[tauri::command]
pub fn set_autostart(app: AppHandle, enabled: bool) {
    let autostart_manager = app.autolaunch();
    if enabled {
        let _ = autostart_manager.enable();
    } else {
        let _ = autostart_manager.disable();
    }
}

#[tauri::command]
pub fn get_autostart(app: AppHandle) -> bool {
    let autostart_manager = app.autolaunch();
    autostart_manager.is_enabled().unwrap()
}
