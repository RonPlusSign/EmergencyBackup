use auto_launch::AutoLaunch;
use std::env;
use std::thread::current;

pub fn install_application() {

    // Get the path of the executable
    let current_exe = env::current_exe().expect("Impossibile ottenere il path dell'eseguibile");
    let mut app_path = current_exe.to_str().unwrap().to_string();

    // Add "" around the path if on Linux, in order to create desktop entry
    if cfg!(target_os = "linux") {
        app_path = format!("\"{}\"", app_path);
    }

    let app_name = "backup_application";
    let auto = AutoLaunch::new(app_name, app_path.as_str(), &[] as &[&str]);

    // If already installed, do nothing
    if auto.is_enabled().unwrap() {
        println!("Auto-launch already configured.");
        return;
    }

    // Enable the auto launch, get error message if it fails
    if let Err(e) = auto.enable() {
        eprintln!("Error during auto-launch configuration: {}", e);
    } else {
        println!("Auto-launch configured correctly.");
    }
}
