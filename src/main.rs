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

use std::{env, thread};
use crate::cpu_log::cpu_logpose;
use crate::installation::install_application;
use crate::pattern_recognition::{wait_for_symbol, Shape};
use rusb::{Context, Device, DeviceHandle, Error, UsbContext};
use crate::sounds::use_audio;
use crate::configuration::{has_shapes_configured, shapes_with_config, Configuration};

fn main() {
    install_application();
    thread::spawn(use_audio);
    thread::spawn(cpu_logpose);
    thread::spawn(|| use_audio("start"));      // TODO: This is only done as a test, remove it when the audio is implemented correctly

    // Create the template shapes that can be recognized
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
                thread::spawn(|| use_audio("start"));
                // TODO: Play sound

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
                        // let mut config = configuration::Configuration::load(format!("config/{}.json", symbol).as_str());
                        let mut config = Configuration::load(symbol).unwrap();
                        config.destination_path = path;
                        thread::spawn(|| use_audio("correct"));
                        // TODO: Play sound?
                        println!("Backup started.");

                        // start backup
                        file::start_backup(config).unwrap();
                    } else {
                        eprintln!("No USB device found. Impossible to start the backup.");
                        thread::spawn(|| use_audio("stop"));
                        // TODO: Play error sound
                        continue;
                    }

                    // TODO: Play sound
                    thread::spawn(|| use_audio("completed"));
                    println!("Backup completed.");
                } else {
                    println!("Backup cancelled.");
                    thread::spawn(|| use_audio("stop"));
                    // TODO: Play sound
                    continue;
                }
            }
        }
    }
}