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

use crate::configuration_gui::ConfigurationGui;
use crate::sounds::use_audio;
use crate::cpu_log::cpu_logpose;
use crate::installation::install_application;
use crate::pattern_recognition::{wait_for_symbol, Shape};
use crate::configuration::{has_shapes_configured, shapes_with_config, Configuration};

fn main() {
    let matches = get_main_matches(); // Set up clap

    // Check for the presence of flags
    if matches.get_flag("config") || !has_shapes_configured() {
        // Open config GUI if no configuration files are present or if requested by the user
        ConfigurationGui::open_window();
        if !has_shapes_configured() {    // If no shapes are configured, exit
            eprintln!("No shapes configured, unable to start backup. Exiting.");
        }
    } else if matches.get_flag("uninstall") {
        installation::uninstall_application();
        return; // Just uninstall the program
    }

    install_application();
    thread::spawn(cpu_logpose);
    thread::spawn(use_audio);      // TODO: This is only done as a test, remove it when the audio is implemented correctly

    // Create the template shapes that can be recognized
    let mut templates = vec![];
    let configured_shapes: Vec<Shape> = shapes_with_config();
    for shape in configured_shapes {
        let shape_templates = Shape::get_templates_for_shape(shape);
        templates.extend(shape_templates);
    }

    loop {
        let symbol = wait_for_symbol(&templates, Arc::new(Mutex::new(false)));
        match symbol {
            None => { return; } // Exit the program if an error occurred
            Some(symbol) => {
                println!("Recognized symbol: {:?}", symbol);
                // TODO: Play sound

                let backup_confirmed = ConfirmationGui::open_window(symbol, Shape::Cross);

                if backup_confirmed { // If same symbol, start the backup
                    //find first usb device available: if found start backup, otherwise show error message
                    if let Some(path) = external_device::get_usb_drive_path() {
                        let mut config = Configuration::load(symbol).unwrap();
                        config.destination_path = path;
                        // TODO: Play sound?
                        println!("Backup started.");

                        // start backup
                        file::start_backup(config).unwrap();
                    } else {
                        eprintln!("No USB device found. Impossible to start the backup.");
                        // TODO: Play error sound
                        continue;
                    }

                    // TODO: Play sound
                    println!("Backup completed.");
                } else {
                    println!("Backup cancelled.");
                    // TODO: Play sound
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