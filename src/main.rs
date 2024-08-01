mod pattern_recognition;
mod configuration;

use crate::pattern_recognition::{draw_shape, wait_for_symbol};

fn main() {
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

    // Create the template shapes for confirmation/rejection of the backup
    let templates_confirmation = vec![
        pattern_recognition::confirm_template(points_per_figure, size),
        pattern_recognition::reject_template(points_per_figure, size),
    ];

    // For debug, show the template shapes as a plot
    // for (i, template) in templates.iter().enumerate().chain(templates_confirmation.iter().enumerate()) {
    //     draw_shape(template.path.clone(), template.name.clone() + i.to_string().as_str() + ".png");
    // }

    loop {
        let symbol = wait_for_symbol(points_per_figure, &templates);
        match symbol {
            None => { return; } // Exit the program if an error occurred
            Some(symbol) => {
                println!("Recognized symbol: {:?}", symbol);
                println!("Please confirm in order to start the backup.");   // TODO: Replace with GUI

                let confirmation = wait_for_symbol(points_per_figure, &templates);
                match confirmation {
                    None => { return; } // Exit the program if an error occurred
                    Some(v) => {
                        // If same symbol, start the backup
                        if v == symbol {
                            println!("Backup started.\n...\n...\n...\nBackup completed.");
                        } else {
                            println!("Backup rejected (found: {:?}).", v);
                            continue;
                        }
                    }

                    /* // If we want to use tick or cross, use this instead of the previous block with Some(v)
                    Some(Shape::Tick) => {
                        println!("Backup started.");

                        // TODO: Load the configuration from a JSON file with the same name as the detected symbol
                        // TODO: if the configuration file does not exist, show an error message and do nothing
                        // let config = Configuration::load("config.json");
                        // println!("Configuration loaded: {:?}", config);

                        // TODO: Backup the files following the configuration
                        // backup(config);
                        println!("Backup completed.");
                    }
                    Some(Shape::Cross) => {
                        println!("Backup rejected.");
                        continue;
                    }
                    Some(_) => { unreachable!("Impossible confirmation symbol."); }
                    */
                }
            }
        }
    }
}