use confirmation_gui::ConfirmationGui;
// #![windows_subsystem = "windows"] // Hide the console window on Windows
use std::sync::{Arc, Mutex};

mod file;
mod cpu_log;
mod sounds;
mod installation;
mod configuration;
mod confirmation_gui;
mod pattern_recognition;
mod external_device;
mod configuration_gui;

use std::thread;
use clap::{Arg, ArgAction, ArgMatches, Command};
use rusb::UsbContext;
use crate::configuration_gui::ConfigurationGui;
use crate::sounds::use_audio;
use crate::cpu_log::cpu_logpose;
use crate::installation::install_application;
use crate::pattern_recognition::{wait_for_symbol, Shape};

fn main() {
    let matches = get_main_matches(); // Set up clap

    // Check for the presence of flags
    if matches.get_flag("config") { // TODO: Open config also if no configuration files are present
        ConfigurationGui::open_window();
    } else if matches.get_flag("uninstall") {
        installation::uninstall_application();
        return;
    }

    install_application();
    thread::spawn(cpu_logpose);
    thread::spawn(use_audio);      // TODO: This is only done as a test, remove it when the audio is implemented correctly

    // Create the template shapes that can be recognized
    // TODO: Insert in the vector the only the templates that have a configuration file
    let templates = vec![
        pattern_recognition::circle_template(false),
        pattern_recognition::circle_template(true),
        pattern_recognition::square_template(false),
        pattern_recognition::square_template(true),
        pattern_recognition::triangle_template(false),
        pattern_recognition::triangle_template(true),
    ];

    loop {
        let symbol = wait_for_symbol(&templates, Arc::new(Mutex::new(false)));
        match symbol {
            None => { return; } // Exit the program if an error occurred
            Some(symbol) => {
                println!("Recognized symbol: {:?}", symbol);

                let backup_confirmed = ConfirmationGui::open_window(symbol, Shape::Cross);

                if backup_confirmed { // If same symbol, start the backup
                    println!("Backup started.\n...\n...\n...");
                    // TODO: Load the configuration from a JSON file with the same name as the detected symbol
                    // TODO: if the configuration file does not exist, show an error message and do nothing
                    // let config = Configuration::load("config.json");
                    // println!("Configuration loaded: {:?}", config);

                    // TODO: Backup the files following the configuration
                    // backup(config);

                    //find first usb device available: if found start backup, otherwise show error message
                    if let Some(path) = external_device::get_usb_drive_path() {
                        let mut config = configuration::Configuration::load(symbol).unwrap();
                        config.destination_path = path;

                        // start backup
                        file::start_backup(config).unwrap();
                    } else {
                        println!("No USB device found. Impossible to start the backup.");
                        //exit and block backup
                        continue;
                    }

                    println!("Backup completed.");
                } else {
                    println!("Backup cancelled.");
                    continue;
                }
            }
        }
    }
}

fn get_main_matches() -> ArgMatches {
    Command::new("EmergencyBackup")
        .version("1.0")
        .author("Andrea Delli (S331998), Andrea Di Battista (S317740), Erika Genova (S332044)")
        .about("A tool for emergency backups")
        .arg(Arg::new("config").long("config").help("Configures the backup").action(ArgAction::SetTrue))
        .arg(Arg::new("uninstall").long("uninstall").help("Uninstalls the program").action(ArgAction::SetTrue))
        .get_matches()
}