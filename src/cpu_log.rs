use std::thread::sleep;
use std::time::Duration;
use sysinfo::{MINIMUM_CPU_UPDATE_INTERVAL, Pid, ProcessesToUpdate, ProcessRefreshKind, System};
fn main() {
    let mut s = System::new_all();
    let cpus = num_cpus::get();
// Wait a bit because CPU usage is based on diff.
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    loop {
// Refresh CPU usage to get actual value.
        s.refresh_processes_specifics(
            ProcessesToUpdate::All,
            ProcessRefreshKind::new().with_cpu()
        );
        if let Some(process) = s.process(Pid::from_u32(std::process::id())) {
            println!("{}% and it is {}", process.cpu_usage()/20.0, cpus);
            sleep(MINIMUM_CPU_UPDATE_INTERVAL);
        }
    }
}