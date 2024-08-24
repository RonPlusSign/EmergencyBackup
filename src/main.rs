use confirmation_gui::ConfirmationGui;
// #![windows_subsystem = "windows"] // Hide the console window on Windows
use std::sync::{Arc, Mutex};

mod file;
mod cpu_log;
mod installation;
mod configuration;
mod confirmation_gui;
mod pattern_recognition;

use std::thread;
use crate::cpu_log::cpu_logpose;
use crate::installation::install_application;
use crate::pattern_recognition::{wait_for_symbol, Shape};

fn main() {
    install_application();
    thread::spawn(cpu_logpose);

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

                let backup_confirmed = ConfirmationGui::open_window(symbol, Shape::Cross);

                if backup_confirmed { // If same symbol, start the backup
                    println!("Backup started.\n...\n...\n...");
                    // TODO: Load the configuration from a JSON file with the same name as the detected symbol
                    // TODO: if the configuration file does not exist, show an error message and do nothing
                    // let config = Configuration::load("config.json");
                    // println!("Configuration loaded: {:?}", config);

                    // TODO: Backup the files following the configuration
                    // backup(config);
                    println!("Backup completed.");
                } else {
                    println!("Backup cancelled.");
                    continue;
                }
            }
        }
    }
}