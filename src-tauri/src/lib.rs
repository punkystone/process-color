use std::{
    collections::HashSet,
    env,
    sync::{Arc, Mutex},
};

use commands::{
    add_process_entry, delete_process_entry, get_autostart, get_mqtt_connection,
    get_process_entrys, get_processes, mqtt_connect, open_config, save_mqtt_connection,
    set_autostart, update_process_entry,
};

use logger::log;
use mqtt::MqttClient;

use crate::setup::setup;
use fetch_processes::fetch_processes;
use process_entry::ProcessEntry;
use storage::Storage;
use tauri::WindowEvent;
use tauri_plugin_autostart::MacosLauncher;

mod commands;
mod fetch_processes;
mod logger;
mod mqtt;
mod mqtt_settings;
mod process_entry;
mod setup;
mod storage;

const AUTO_START: &str = "--autostart";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let storage = Arc::new(Mutex::new(Storage::new()));
    let processes = Arc::new(Mutex::new(HashSet::<String>::new()));
    let process_entrys = Arc::new(Mutex::new(Vec::<ProcessEntry>::new()));
    let mqtt_client = Arc::new(Mutex::new(MqttClient::new()));

    let error = tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![AUTO_START]),
        ))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(processes.clone())
        .manage(process_entrys.clone())
        .manage(mqtt_client.clone())
        .manage(storage.clone())
        .invoke_handler(tauri::generate_handler![
            get_processes,
            update_process_entry,
            get_process_entrys,
            mqtt_connect,
            add_process_entry,
            delete_process_entry,
            open_config,
            get_mqtt_connection,
            save_mqtt_connection,
            set_autostart,
            get_autostart
        ])
        .setup(move |app| {
            setup(app, storage, process_entrys, mqtt_client, processes);
            Ok(())
        })
        .on_window_event(|window, event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                let result = window.hide();
                if result.is_err() {
                    log("failed to hide window");
                }
            }
            _ => (),
        })
        .run(tauri::generate_context!());
    if error.is_err() {
        eprintln!("error starting app");
    }
}
