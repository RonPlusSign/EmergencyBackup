use std::path::PathBuf;
use eframe::App;
use eframe::emath::{Align, Vec2};
use crate::configuration::Configuration;
use crate::pattern_recognition::{Shape};
use rfd::FileDialog;

/* Configuration window, where the user can set the shape, source path, and optional extension filter.
Show a title, at the top and then 2 columns:
 - Left column: 3 input fields: shape (dropdown), source path (egui files), and extension filter
 - Right column: gif preview of the selected shape
 At the bottom right, show a button to close and another to save the configuration (disabled if fields are missing).
 When the shape is changed, the configuration of the shape is loaded from a JSON file with the same name as the shape (if exists).
 On save, the configuration is saved to a JSON file with the same name as the shape.
 */


pub struct ConfigurationGui {
    shape: Shape,               // Shape to set the configuration
    path: PathBuf,              // Source path
    extension_filter: String,   // Extension filter
}

impl App for ConfigurationGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);    // To render images correctly

        // Render the GUI
        egui::CentralPanel::default().show(ctx, |ui| {
            // Align everything to the center
            ui.vertical_centered(|ui| {
                ui.allocate_space(Vec2::new(0.0, 20.0)); // Add some space between the title and the symbols
                ui.heading("Emergency Backup configuration");
                ui.label("Select the shape, source path, and optional extension filter.");

                // Horizontal layout for the symbols
                ui.horizontal_centered(|ui| {
                    ui.columns(2, |columns| {
                        let max_height = 180.0;

                        columns[0].with_layout(egui::Layout::top_down(Align::Center), |ui| {
                            ui.allocate_space(Vec2::new(0.0, 20.0)); // Add some space between the title and the symbols
                            ui.heading("Configuration");

                            // Dropdown for the shape
                            ui.horizontal(|ui| {
                                ui.label("Shape:");

                                egui::ComboBox::from_label("Shape")
                                    .selected_text(self.shape.to_string())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut self.shape, Shape::Circle, "Circle");
                                        ui.selectable_value(&mut self.shape, Shape::Square, "Square");
                                        ui.selectable_value(&mut self.shape, Shape::Triangle, "Triangle");
                                    });
                            });

                            // Source path
                            ui.horizontal(|ui| {
                                ui.label("Source path:");
                                if ui.button("Select file...").clicked() {
                                    if let Some(path) = FileDialog::new().pick_folder() {
                                        self.path = path;
                                    }
                                }
                            });

                            // Extension filter
                            ui.horizontal(|ui| {
                                ui.label("Extension filter:");
                                ui.text_edit_singleline(&mut self.extension_filter);
                            });
                        });

                        columns[1].with_layout(egui::Layout::top_down(Align::Center), |ui| {
                            ui.allocate_space(Vec2::new(0.0, 20.0)); // Add some space between the title and the symbols
                            ui.centered_and_justified(|ui| {
                                match self.shape {
                                    Shape::Circle => ui.add(egui::Image::new(egui::include_image!("../images/circle.gif")).max_height(max_height)),
                                    Shape::Square => ui.add(egui::Image::new(egui::include_image!("../images/square.gif")).max_height(max_height)),
                                    Shape::Triangle => ui.add(egui::Image::new(egui::include_image!("../images/triangle.gif")).max_height(max_height)),
                                    _ => { ui.label("Invalid shape.") }
                                }
                            });
                        });
                    });

                    // Save button
                    if ui.button("Save").clicked() {
                        let config = Configuration::new(self.shape,
                                                        self.path.to_str().unwrap().to_string(),
                                                        "".to_string(),
                                                        if self.extension_filter.is_empty() { None } else { Some(self.extension_filter.clone()) });
                        config.save(format!("config/{}.json", self.shape).as_str());
                    }

                    // Close button
                    if ui.button("Close").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });
    }
}

impl ConfigurationGui {
    /// Open the configuration window. Returns true if there's at least one configuration saved, false otherwise.
    pub fn open_window() -> bool {
        let gui = ConfigurationGui { shape: Shape::Circle, path: PathBuf::new(), extension_filter: String::new() };

        let (width, height) = (700.0, 500.0);
        let native_options = eframe::NativeOptions {
            follow_system_theme: true,  // Note: currently not switching themes on Linux (see NativeOptions docs)
            centered: true, // Note: currently not supported by Wayland (see NativeOptions docs)
            viewport: egui::ViewportBuilder::default()
                .with_min_inner_size([width, height])
                .with_max_inner_size([width, height])
                .with_resizable(false)
                .with_icon(eframe::icon_data::from_png_bytes(&include_bytes!("../images/emergency-backup-icon.png")[..]).expect("Failed to load icon")),
            ..Default::default()
        };

        // Run the GUI
        eframe::run_native("Configure Emergency Backup", native_options, Box::new(|_cc| Ok(Box::new(gui)))).expect("Failed to run the GUI");

        // TODO: Return true if there's at least one configuration saved, false otherwise
        false
    }
}
