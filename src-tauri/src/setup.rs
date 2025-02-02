use std::{
    collections::HashSet,
    env,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    App, Emitter, Manager,
};

use crate::{
    fetch_processes, log, logger::set_log_path, mqtt::MqttClient, process_entry::ProcessEntry,
    storage::Storage, AUTO_START,
};

pub fn setup(
    app: &mut App,
    storage: Arc<Mutex<Storage>>,
    process_entrys: Arc<Mutex<Vec<ProcessEntry>>>,
    mqtt_client: Arc<Mutex<MqttClient>>,
    processes: Arc<Mutex<HashSet<String>>>,
) {
    let app_data_dir = app.path().app_data_dir();
    if app_data_dir.is_err() {
        return;
    }
    let path = app_data_dir.unwrap().into_os_string().into_string();
    if path.is_err() {
        return;
    }
    let path = path.unwrap();
    {
        let storage_lock = storage.lock();
        if storage_lock.is_err() {
            return;
        }
        storage_lock.unwrap().set_path(path.clone());
        set_log_path(path);
    }
    log("test");
    let args: Vec<String> = env::args().collect();
    if args.contains(&AUTO_START.to_owned()) {
        let window = app.get_webview_window("main");
        if window.is_none() {
            log("main window not found");
            return;
        } else {
            if let Err(_) = window.unwrap().hide() {
                log("failed to hide main window");
            }
        }
    }

    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>);
    if quit_item.is_err() {
        log("failed to create quit item");
        return;
    }
    let quit_item = quit_item.unwrap();

    let open_item = MenuItem::with_id(app, "open", "Open", true, None::<&str>);
    if open_item.is_err() {
        log("failed to create open item");
        return;
    }
    let open_item = open_item.unwrap();

    let menu = Menu::with_items(app, &[&open_item, &quit_item]);
    if menu.is_err() {
        log("failed to create menu");
        return;
    }
    let menu = menu.unwrap();

    let storage_system_tray_clone = storage.clone();
    let process_entrys_system_tray_clone = process_entrys.clone();

    let icon = app.default_window_icon();
    if icon.is_none() {
        log("failed to get default window icon");
        return;
    }
    let icon = icon.unwrap();

    let result = TrayIconBuilder::new()
        .menu(&menu)
        .show_menu_on_left_click(false)
        .icon(icon.clone())
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "quit" => {
                let storage_system_tray_clone = storage_system_tray_clone.lock();
                if storage_system_tray_clone.is_err() {
                    log("failed to lock storage system tray");
                    return;
                }
                let process_entrys_system_tray_clone = process_entrys_system_tray_clone.lock();
                if process_entrys_system_tray_clone.is_err() {
                    log("failed to lock process entrys system tray");
                    return;
                }
                storage_system_tray_clone
                    .unwrap()
                    .save_process_entrys(&process_entrys_system_tray_clone.unwrap());

                app.exit(0);
            }
            "open" => {
                let window = app.get_webview_window("main");
                if window.is_some() {
                    let result = window.unwrap().show();
                    if result.is_err() {
                        log("failed to show window");
                    }
                } else {
                    log("failed to get webview window");
                }
            }
            _ => {}
        })
        .build(app);

    if result.is_err() {
        log("failed to build system tray");
    }

    {
        let storage = storage.lock();
        let process_entrys = process_entrys.lock();
        let mqtt_client = mqtt_client.lock();

        if storage.is_err() {
            log("failed to lock storage");
            return;
        }
        if process_entrys.is_err() {
            log("failed to lock process entrys");
            return;
        }
        if mqtt_client.is_err() {
            log("failed to lock mqtt client");
            return;
        }

        let storage = storage.unwrap();
        let mut mqtt_client = mqtt_client.unwrap();

        let saved_process_entrys = storage.get_saved_process_entrys();
        *process_entrys.unwrap() = saved_process_entrys;

        let saved_mqtt_settings = storage.get_mqtt_settings();
        mqtt_client.settings = Some(saved_mqtt_settings);
        mqtt_client.connect();
    }

    let running_states_app_handle = app.handle().clone();
    let mqtt_connction_state_app_handle = app.handle().clone();

    let mqtt_client_connected = mqtt_client.clone();
    let processes_clone = processes.clone();

    thread::spawn(|| fetch_processes(processes));
    thread::spawn(move || loop {
        {
            let process_entrys = process_entrys.lock();
            if process_entrys.is_err() {
                log("failed to lock process entrys");
                continue;
            }
            let mut process_entrys = process_entrys.unwrap();

            let mqtt_client = mqtt_client.lock();
            if mqtt_client.is_err() {
                log("failed to lock mqtt client");
                continue;
            }
            let mqtt_client = mqtt_client.unwrap();

            let processes = processes_clone.lock();
            if processes.is_err() {
                log("failed to lock processes");
                continue;
            }
            let processes = processes.unwrap();

            for entry in process_entrys.iter_mut() {
                if processes.contains(&entry.name) && !entry.is_running {
                    entry.is_running = true;
                    mqtt_client.publish(&entry.topic, &entry.value);
                    mqtt_client.publish(
                        &"tgn/esp_3/neopixel/brightness".to_string(),
                        &"150".to_string(),
                    );
                } else if !processes.contains(&entry.name) && entry.is_running {
                    entry.is_running = false;
                    mqtt_client.publish(&entry.topic, &entry.off_value);
                }
            }

            let running_states: Vec<bool> = (*process_entrys)
                .iter()
                .map(|entry| entry.is_running)
                .collect();

            let error = running_states_app_handle.emit("running_states", running_states);
            if error.is_err() {
                log("failed to emit running_states");
                continue;
            }
        }
        thread::sleep(Duration::from_secs(1));
    });

    thread::spawn(move || loop {
        {
            let mqtt_client = mqtt_client_connected.lock();
            if mqtt_client.is_err() {
                log("failed to lock mqtt client");
                continue;
            }
            let connected = mqtt_client.unwrap().is_connected();
            let error = mqtt_connction_state_app_handle.emit("mqtt_connection_state", connected);
            if error.is_err() {
                log("failed to emit mqtt_connection_state");
                continue;
            }
        }
        thread::sleep(Duration::from_secs(1));
    });
}
