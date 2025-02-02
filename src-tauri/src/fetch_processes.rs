use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System};

pub fn fetch_processes(processes: Arc<Mutex<HashSet<String>>>) {
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::nothing()),
    );

    loop {
        sys.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::nothing(),
        );
        let mut current_processes = HashSet::<String>::new();

        for (_, process) in sys.processes() {
            if let Some(name) = process.name().to_str() {
                current_processes.insert(name.to_string());
            }
        }
        {
            let processes = processes.lock();
            if processes.is_err() {
                continue;
            }
            let mut processes = processes.unwrap();
            *processes = current_processes;
        }
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    }
}
