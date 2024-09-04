use crate::configuration::Configuration;
use crate::pattern_recognition::Shape;
use eframe::emath::Align;
use eframe::App;
use egui::{Layout, Vec2};
use egui_extras::install_image_loaders;
use rfd::FileDialog;
use std::path::PathBuf;

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
        install_image_loaders(ctx); // Ensure images load correctly
        let selected_shape = self.shape;

        // Render the UI
        egui::CentralPanel::default().show(ctx, |ui| {
            // Top section: Title and description
            ui.vertical_centered(|ui| {
                ui.add_space(20.0); // Add space above the title
                ui.heading("Emergency Backup Configuration");
                ui.add_space(10.0);
                ui.label("Select the shape, source path, and optional extension filter.");
                ui.add_space(20.0);
            });

            // Get the available width and calculate the proportions
            let available_width = ui.available_width();
            let column1_width = available_width * (2.0 / 3.0);
            let column2_width = available_width * (1.0 / 3.0);
            // Main content with 2 columns
            ui.horizontal(|ui| {
                // Left column: Configuration fields
                ui.allocate_ui_with_layout(Vec2::new(column1_width, ui.available_height()), Layout::top_down(Align::Center), |ui| {
                    ui.vertical_centered(|ui| { ui.heading("Configuration"); });
                    ui.add_space(10.0);

                    // Shape dropdown
                    ui.horizontal(|ui| {
                        ui.label("Shape:");
                        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                            egui::ComboBox::from_label("")
                                .selected_text(self.shape.to_string())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.shape, Shape::Circle, "Circle");
                                    ui.selectable_value(&mut self.shape, Shape::Square, "Square");
                                    ui.selectable_value(&mut self.shape, Shape::Triangle, "Triangle");
                                });
                        });
                    });

                    ui.add_space(10.0);

                    // Source path selection
                    ui.horizontal(|ui| {
                        ui.label("Source Path:");

                        // Display the selected path (truncate if too long)
                        let displayed_path = self.path.to_str().unwrap_or("No folder selected");
                        let max_length = 30; // Set max length for displayed path

                        let displayed_path = if displayed_path.len() > max_length {
                            format!("...{}", &displayed_path[displayed_path.len() - max_length..])
                        } else if displayed_path.is_empty() {
                            "No folder selected".to_string()
                        } else {
                            displayed_path.to_string()
                        };

                        // Show the selected path in a label
                        ui.label(displayed_path);

                        // Button to select a new folder
                        if ui.button("Select Folder...").clicked() {
                            if let Some(path) = FileDialog::new().pick_folder() {
                                self.path = path;
                            }
                        }
                    });


                    ui.add_space(10.0);

                    // Extension filter input
                    ui.horizontal(|ui| {
                        ui.label("Extension Filter:");
                        ui.text_edit_singleline(&mut self.extension_filter);
                    });

                    ui.end_row(); // End of the left column
                });

                // Right column: GIF preview of the shape
                ui.allocate_ui_with_layout(Vec2::new(column2_width, ui.available_height()), Layout::top_down(Align::Center), |ui| {

                    // ui.vertical_centered_justified(|ui| {
                    let max_height = 180.0;
                    match self.shape {
                        Shape::Circle => { ui.add(egui::Image::new(egui::include_image!("../images/circle.gif")).max_height(max_height)); }
                        Shape::Square => { ui.add(egui::Image::new(egui::include_image!("../images/square.gif")).max_height(max_height)); }
                        Shape::Triangle => { ui.add(egui::Image::new(egui::include_image!("../images/triangle.gif")).max_height(max_height)); }
                        _ => {}
                    }
                });

                ui.end_row(); // End of the right column
            });

            // Footer buttons (Save & Close)
            ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                // Close button
                if ui.button("Close").clicked() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }

                // Save button (enabled only when all fields are filled, except for the extension filter which is optional)
                let save_enabled = !self.path.to_str().unwrap_or("").is_empty();

                if ui.add_enabled(save_enabled, egui::Button::new("Save")).clicked() {
                    let config = Configuration::new(
                        self.shape,
                        self.path.to_str().unwrap().to_string(),
                        "".to_string(),
                        if self.extension_filter.is_empty() { None } else { Some(self.extension_filter.clone()) },
                    );
                    config.save();
                }

                ui.add_space(10.0);
                ui.end_row();
            });
        });

        if selected_shape != self.shape {
            self.reload_configuration();
        }
    }
}

impl ConfigurationGui {
    /// Open the configuration window
    pub fn open_window() {
        // Load the default configuration or create an empty one
        let default_config = Configuration::load(Shape::Circle);
        let gui = if let Some(config) = default_config {
            ConfigurationGui { shape: config.shape, path: PathBuf::from(config.source_path), extension_filter: config.extension_filter.unwrap_or_default() }
        } else { ConfigurationGui { shape: Shape::Circle, path: PathBuf::new(), extension_filter: String::new() } };

        let (width, height) = (700.0, 300.0);
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
    }

    /// Reload the configuration for the current shape
    ///
    /// If the configuration file does not exist, the fields are cleared.
    /// If the configuration file exists, the fields are filled with the values in the file.
    fn reload_configuration(&mut self) {
        let config: Option<Configuration> = Configuration::load(self.shape);
        if let Some(config) = config {
            self.path = PathBuf::from(config.source_path);
            self.extension_filter = config.extension_filter.unwrap_or_default();
        } else {
            self.path = PathBuf::new();
            self.extension_filter = String::new();
        }
    }
}
