use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use sysinfo::{Pid, ProcessesToUpdate, ProcessRefreshKind, System};

pub fn cpu_logpose() -> Result<(), std::io::Error>{
    let mut s = System::new_all();
    let cpus: f32 = num_cpus::get() as f32;
    let mut cpu_single_use: f32 = 0.0;
    let mut count = 0.0;
    let mut cpu_use = 0.0;
    let mut num_cycles = 0;
    loop {
        // Refresh CPU usage to get actual value.
        s.refresh_processes_specifics(
            ProcessesToUpdate::All,
            ProcessRefreshKind::new().with_cpu(),
        );
        //this will count the total cpu usage of the current process
        if let Some(process) = s.process(Pid::from_u32(std::process::id())) {
            sleep(Duration::from_secs(5));
            cpu_single_use += process.cpu_usage() / cpus;
            count += 1.0;
        }

        if count == 2.0 {
            cpu_use = cpu_single_use / count;
            num_cycles += 1;
            cpu_logfile(num_cycles, cpu_use)?;
            count = 0.0;
            cpu_single_use = 0.0;

        }
    }

}

fn cpu_logfile(n:i32, c:f32) -> std::io::Result<()> {
    // Specify the file path
    let file_path = "cpu_logfile.txt";

    // Open the file in append mode, create it if it doesn't exist
    let mut file = OpenOptions::new()
        .append(true)  // Append mode
        .create(true)  // Create the file if it doesn't exist
        .open(file_path)?;

    // Get the current local time
    let now = chrono::Local::now();

    // Format the timestamp (e.g., "2024-08-22 14:35:06")
    let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

    // Write the formatted string to the file
    writeln!(file, "[{}]: percentage: {}", timestamp, c)?;

    Ok(())
}