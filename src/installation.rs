use auto_launch::AutoLaunch;
use std::env;

pub fn install_application() {

    // Get the path of the executable
    let current_exe = env::current_exe().expect("Impossibile ottenere il path dell'eseguibile");
    let app_name = "backup_application";
    let app_path = current_exe.to_str().unwrap();
    let auto = AutoLaunch::new(app_name, app_path, &[] as &[&str]);

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
