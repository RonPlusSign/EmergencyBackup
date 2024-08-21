use std::thread::sleep;
use std::time::Duration;
use sysinfo::{Pid, ProcessesToUpdate, ProcessRefreshKind, System};

pub fn cpu_logpose() {
    let mut s = System::new_all();
    let cpus: f32 = num_cpus::get() as f32;
    let mut cpu_single_use: f32 = 0.0;
    let mut count = 0.0;
    let mut cpu_use = 0.0;
    loop {
        // Refresh CPU usage to get actual value.
        s.refresh_processes_specifics(
            ProcessesToUpdate::All,
            ProcessRefreshKind::new().with_cpu(),
        );
        //this will count the total cpu usage of the current process
        if let Some(process) = s.process(Pid::from_u32(std::process::id())) {
            println!("{}% and it is {}", process.cpu_usage() / cpus, cpus);
            sleep(Duration::from_secs(5));
            cpu_single_use += process.cpu_usage() / cpus;
            count += 1.0;
        }

        if count == 24.0 {
            cpu_use = cpu_single_use / count;
            println!("take this total {}% !!", cpu_use);
            count = 0.0;
        }
    }
}