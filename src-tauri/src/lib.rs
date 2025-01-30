use std::{
    collections::HashSet,
    env,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use commands::{
    add_process_entry, delete_process_entry, get_autostart, get_mqtt_connection,
    get_process_entrys, get_processes, mqtt_connect, open_config, save_mqtt_connection,
    set_autostart, update_process_entry,
};
use mqtt::MqttClient;

use process_entry::ProcessEntry;
use fetch_processes::fetch_processes;
use storage::Storage;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Emitter, Manager, WindowEvent,
};
use tauri_plugin_autostart::MacosLauncher;

mod commands;
mod mqtt;
mod mqtt_settings;
mod process_entry;
mod fetch_processes;
mod storage;

const AUTO_START: &str = "--autostart";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let storage = Arc::new(Mutex::new(Storage::new()));
    let processes = Arc::new(Mutex::new(HashSet::<String>::new()));
    let process_entrys = Arc::new(Mutex::new(Vec::<ProcessEntry>::new()));
    let mqtt_client = Arc::new(Mutex::new(MqttClient::new()));

    let processes_clone = processes.clone();
    let mqtt_client_connected = mqtt_client.clone();

    tauri::Builder::default()
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
            let args: Vec<String> = env::args().collect();
            if args.contains(&AUTO_START.to_owned()) {
                let _ = app.get_webview_window("main").unwrap().hide();
            }

            storage.lock().unwrap().set_path(
                app.path()
                    .app_data_dir()
                    .unwrap()
                    .into_os_string()
                    .into_string()
                    .unwrap(),
            );

            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let open_item = MenuItem::with_id(app, "open", "Open", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&open_item, &quit_item])?;
            let storage_system_tray_clone = storage.clone();
            let process_entrys_system_tray_clone = process_entrys.clone();

            TrayIconBuilder::new()
                .menu(&menu)
                .show_menu_on_left_click(false)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "quit" => {
                        storage_system_tray_clone
                            .lock()
                            .unwrap()
                            .save_process_entrys(&process_entrys_system_tray_clone.lock().unwrap());

                        app.exit(0);
                    }
                    "open" => {
                        let _ = app.get_webview_window("main").unwrap().show();
                    }
                    _ => {}
                })
                .build(app)?;

            let saved_process_entrys = storage.lock().unwrap().get_saved_process_entrys();
            *process_entrys.lock().unwrap() = saved_process_entrys;

            let saved_mqtt_settings = storage.lock().unwrap().get_mqtt_settings();
            mqtt_client.lock().unwrap().settings = Some(saved_mqtt_settings);
            mqtt_client.lock().unwrap().connect();

            let running_states_app_handle = app.handle().clone();
            let mqtt_connction_state_app_handle = app.handle().clone();

            thread::spawn(|| fetch_processes(processes_clone));
            thread::spawn(move || loop {
                {
                    let mut process_entrys = process_entrys.lock().unwrap();
                    let processes = processes.lock().unwrap();
                    for entry in process_entrys.iter_mut() {
                        if processes.contains(&entry.name) && !entry.is_running {
                            entry.is_running = true;
                            mqtt_client
                                .lock()
                                .unwrap()
                                .publish(&entry.topic, &entry.value);
                            mqtt_client.lock().unwrap().publish(
                                &"tgn/esp_3/neopixel/brightness".to_string(),
                                &"150".to_string(),
                            );
                        } else if !processes.contains(&entry.name) && entry.is_running {
                            entry.is_running = false;
                            mqtt_client
                                .lock()
                                .unwrap()
                                .publish(&entry.topic, &entry.off_value);
                        }
                    }

                    let running_states: Vec<bool> = (*process_entrys)
                        .iter()
                        .map(|entry| entry.is_running)
                        .collect();

                    running_states_app_handle
                        .emit("running_states", running_states)
                        .unwrap();
                }
                thread::sleep(Duration::from_secs(1));
            });
            thread::spawn(move || loop {
                {
                    let connected = mqtt_client_connected.lock().unwrap().is_connected();
                    mqtt_connction_state_app_handle
                        .emit("mqtt_connection_state", connected)
                        .unwrap();
                }
                thread::sleep(Duration::from_secs(1));
            });
            Ok(())
        })
        .on_window_event(|window, event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                let _ = window.hide();
            }
            _ => (),
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
