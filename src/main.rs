#![windows_subsystem = "windows"] // Hide the console window on Windows
mod pattern_recognition;
mod configuration;
mod file;
mod confirmation_egui;
mod cpu_log;
mod installation;

use crate::installation::install_application;
use crate::pattern_recognition::{draw_shape, Shape, wait_for_symbol};

fn main() {
    install_application();

    let size = 100.0;
    let points_per_figure = 200;   // Maximum number of points to store

    // Create the template shapes that can be recognized
    let templates = vec![
        pattern_recognition::circle_template(points_per_figure, size, false),
        pattern_recognition::circle_template(points_per_figure, size, true),
        pattern_recognition::square_template(points_per_figure, size, false),
        pattern_recognition::square_template(points_per_figure, size, true),
        pattern_recognition::triangle_template(points_per_figure, size, false),
        pattern_recognition::triangle_template(points_per_figure, size, true),
    ];

    // For debug, show the template shapes as a plot
    // for (i, template) in templates.iter().enumerate() {
    //     draw_shape(template.path.clone(), template.name.clone() + i.to_string().as_str() + ".png");
    // }

    loop {
        let symbol = wait_for_symbol(points_per_figure, &templates);
        match symbol {
            None => { return; } // Exit the program if an error occurred
            Some(symbol) => {
                println!("Recognized symbol: {:?}", symbol);
                println!("Please confirm in order to start the backup.");   // TODO: Replace with GUI

                let confirm_templates = vec![Shape::get_templates_for_shape(symbol, size, points_per_figure),   // Confirm by redrawing the same symbol
                                             Shape::get_templates_for_shape(Shape::Cross, size, points_per_figure)] // Reject by drawing an X
                    .into_iter().flat_map(|x| x.into_iter()).collect();
                let confirmation = wait_for_symbol(points_per_figure, &confirm_templates);
                match confirmation {
                    None => { return; } // Exit the program if an error occurred
                    Some(v) => {
                        if v == symbol { // If same symbol, start the backup
                            println!("Backup started.\n...\n...\n...");
                            // TODO: Load the configuration from a JSON file with the same name as the detected symbol
                            // TODO: if the configuration file does not exist, show an error message and do nothing
                            // let config = Configuration::load("config.json");
                            // println!("Configuration loaded: {:?}", config);

                            // TODO: Backup the files following the configuration
                            // backup(config);
                            println!("Backup completed.");
                        } else {
                            println!("Backup rejected (found: {:?}).", v);
                            continue;
                        }
                    }
                }
            }
        }
    }
}