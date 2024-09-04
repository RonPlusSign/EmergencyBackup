use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};

pub fn cpu_logpose() -> Result<(), std::io::Error> {
    let mut s = System::new_all();
    let cpus: f32 = num_cpus::get() as f32;
    let mut cpu_single_use: f32 = 0.0;
    let mut count = 0.0;
    let mut cpu_use:f32;
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

        if count == 24.0 {  // 5 seconds * 24 = 2 minutes
            cpu_use = cpu_single_use / count;
            cpu_logfile(cpu_use)?;
            count = 0.0;
            cpu_single_use = 0.0;
        }
    }
}

fn cpu_logfile(cpu_usage: f32) -> std::io::Result<()> {
    // Specify the file path
    let mut file_path = env::current_exe()?.parent().unwrap().to_path_buf();

    // create the directory if it doesn't exist
    std::fs::create_dir_all(file_path.clone())?;

    // add the directory to the path
    file_path.push("cpu_logfile.txt");

    //you will find where the log file is saved
    // println!("Current directory: {:?}", file_path);

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
    writeln!(file, "[{}] CPU: {}%", timestamp, cpu_usage)?;

    Ok(())
}