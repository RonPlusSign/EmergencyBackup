use std::sync::{Arc, Mutex};
use eframe;
use eframe::App;
use eframe::emath::Align;
use eframe::glow::Context;
use egui;
use egui::Vec2;
use guessture::Template;
use crate::pattern_recognition::{Shape, wait_for_symbol};

/* Confirmation dialog in order to start the backup.
Show a title, and then 2 texts side to side: "Confirm" and "Cancel".
After the user redraws the symbol, the function returns true if the symbol is the same as the one recognized before, false otherwise,
and the window is closed. */

pub struct ConfirmationGui {
    status: Arc<Mutex<Option<bool>>>, // None = still waiting, Some(true) = confirmed, Some(false) = cancelled
    confirm_shape: Shape,             // Shape to confirm the backup
    cancel_shape: Shape,              // Shape to cancel the backup
}

impl App for ConfirmationGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);    // To render images correctly

        let status = self.status.lock().unwrap(); // Get the GUI status

        // Render the GUI
        egui::CentralPanel::default().show(ctx, |ui| {
            // Align everything to the center
            ui.vertical_centered(|ui| {
                ui.allocate_space(Vec2::new(0.0, 20.0)); // Add some space between the title and the symbols
                ui.heading("Redraw the symbol to confirm or cancel.");

                // Horizontal layout for the symbols
                ui.horizontal_centered(|ui| {
                    ui.columns(2, |columns| {
                        let max_height = 180.0;

                        columns[0].with_layout(egui::Layout::top_down(Align::Center), |ui| {
                            ui.allocate_space(Vec2::new(0.0, 20.0)); // Add some space between the title and the symbols
                            ui.heading(format!("Confirm: {}", self.confirm_shape));

                            ui.centered_and_justified(|ui| {
                                match self.confirm_shape {
                                    Shape::Circle => ui.add(egui::Image::new(egui::include_image!("../images/circle.gif")).max_height(max_height)),
                                    Shape::Square => ui.add(egui::Image::new(egui::include_image!("../images/square.gif")).max_height(max_height)),
                                    Shape::Triangle => ui.add(egui::Image::new(egui::include_image!("../images/triangle.gif")).max_height(max_height)),
                                    _ => { ui.label("Invalid shape.") }
                                }
                            });
                        });

                        columns[1].with_layout(egui::Layout::top_down(Align::Center), |ui| {
                            ui.allocate_space(Vec2::new(0.0, 20.0)); // Add some space between the title and the symbols
                            ui.heading(format!("Cancel: {}", self.cancel_shape));
                            ui.centered_and_justified(|ui| {
                                ui.add(egui::Image::new(egui::include_image!("../images/cancel.gif")).max_height(max_height));
                            });
                        });
                    });
                });
            });
        });

        // Close the window if the status is set
        if let Some(_) = *status { ctx.send_viewport_cmd(egui::ViewportCommand::Close); }
    }

    fn on_exit(&mut self, _gl: Option<&Context>) {
        let mut status = self.status.lock().unwrap();
        if status.is_none() { *status = Some(false); } // Cancel the backup if the window is closed
    }
}

impl ConfirmationGui {
    pub fn open_window(confirm_shape: Shape, cancel_shape: Shape) -> bool {
        let gui = ConfirmationGui { confirm_shape, cancel_shape, status: Arc::new(Mutex::new(None)) };
        let gui_status = gui.status.clone();
        let gui_status_thread = gui.status.clone();
        let stop = Arc::new(Mutex::new(false));
        let stop_thread = stop.clone();

        let handle = std::thread::spawn(move || {
            let confirm_templates: Vec<Template> = vec![Shape::get_templates_for_shape(confirm_shape),   // Confirm by redrawing the same symbol
                                                        Shape::get_templates_for_shape(cancel_shape)] // Cancel by drawing an X
                .into_iter().flat_map(|x| x.into_iter()).collect();
            let confirmation = wait_for_symbol(&confirm_templates, stop_thread);

            let mut status = gui_status_thread.lock().unwrap();
            match confirmation {
                None => { *status = Some(false); } // Cancel the backup if an error occurred
                Some(v) => { *status = Some(v == confirm_shape); } // Confirm the backup by redrawing the same symbol
            };
        });

        let (width, height) = (700.0, 500.0);
        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_min_inner_size([width, height])
                .with_max_inner_size([width, height])
                .with_position([(1920.0 - width) / 2.0, (1080.0 - height) / 2.0])
                .with_resizable(false)
                .with_icon(eframe::icon_data::from_png_bytes(&include_bytes!("../images/emergency-backup-icon.png")[..])
                    .expect("Failed to load icon")),
            ..Default::default()
        };

        // Run the GUI
        eframe::run_native("Confirm backup", native_options, Box::new(|_cc| Ok(Box::new(gui)))).expect("Failed to run the GUI");

        // If the window is closed manually, stop the recognition thread (if it's still running)
        let mut stop = stop.lock().unwrap();
        *stop = true;
        drop(stop); // Drop the mutex to unlock it for the other thread
        handle.join().unwrap();

        let status = gui_status.lock().unwrap();
        status.unwrap()
    }
}