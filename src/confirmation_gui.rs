use std::sync::{Arc, Mutex};
use eframe;
use eframe::App;
use eframe::emath::Align;
use eframe::glow::Context;
use egui;
use guessture::Template;
use crate::pattern_recognition::{Shape, wait_for_symbol};

/* Confirmation dialog in order to start the backup.
Show a title, and then 2 texts side to side: "Confirm" and "Cancel".
After the user redraws the symbol, the function returns true if the symbol is the same as the one recognized before, false otherwise,
and the window is closed. */

struct ConfirmationStatus {
    confirm_shape: Shape,       // Shape to confirm the backup
    cancel_shape: Shape,        // Shape to cancel the backup
    status: Option<bool>,   // None = still waiting, Some(true) = confirmed, Some(false) = cancelled
}

pub struct ConfirmationGui {
    status: Arc<Mutex<ConfirmationStatus>>,
}

impl App for ConfirmationGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let status = self.status.lock().unwrap();
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                // Align everything to the center
                ui.with_layout(egui::Layout::top_down_justified(Align::Center), |ui| {
                    ui.heading("Redraw the symbol to confirm or cancel.");

                    // Horizontal layout for the symbols
                    ui.horizontal(|ui| {
                        // Confirm symbol
                        ui.vertical(|ui| {
                            ui.heading("Confirm:");
                            // URI = "../images/shape.gif"
                            ui.image(egui::include_image!("../images/circle.gif"));
                            ui.label(status.confirm_shape.to_string());
                        });

                        ui.separator();

                        // Cancel symbol
                        ui.vertical(|ui| {
                            ui.heading("Cancel:");
                            // URI = "../images/shape.gif"
                            ui.image(egui::include_image!("../images/cancel.gif"));
                            ui.label(status.cancel_shape.to_string());
                        });
                    });
                });
            });

        // Close the window if the status is set
        if let Some(_) = status.status {
            println!("Closing the window.");
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }

    fn on_exit(&mut self, _gl: Option<&Context>) {
        let mut status = self.status.lock().unwrap();
        status.status = Some(false);    // Cancel the backup if the window is closed
    }
}

impl ConfirmationGui {
    pub fn open_window(confirm_shape: Shape, cancel_shape: Shape) -> bool {
        let gui = ConfirmationGui { status: Arc::new(Mutex::new(ConfirmationStatus { confirm_shape, cancel_shape, status: None })) };
        let gui_status = gui.status.clone();
        let gui_status_thread = gui.status.clone();

        let handle = std::thread::spawn(move || {
            let confirm_templates: Vec<Template> = vec![Shape::get_templates_for_shape(confirm_shape),   // Confirm by redrawing the same symbol
                                                        Shape::get_templates_for_shape(Shape::Cross)] // Cancel by drawing an X
                .into_iter().flat_map(|x| x.into_iter()).collect();
            let confirmation = wait_for_symbol(&confirm_templates);

            let mut status = gui_status_thread.lock().unwrap();
            match confirmation {
                None => { status.status = Some(false); } // Cancel the backup if an error occurred
                Some(v) => { status.status = Some(v == confirm_shape); } // Confirm the backup by redrawing the same symbol
            };
        });

        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_min_inner_size([800.0, 500.0])
                .with_max_inner_size([800.0, 500.0])
                .with_position([560.0, 290.0])  // Center the window (800x500) on a 1920x1080 screen
                .with_resizable(false)
                .with_icon(eframe::icon_data::from_png_bytes(&include_bytes!("../images/emergency-backup-icon.png")[..])
                    .expect("Failed to load icon")),
            ..Default::default()
        };

        // Run the GUI
        eframe::run_native("Confirm backup", native_options, Box::new(|cc| Ok(Box::new(gui)))).expect("Failed to run the GUI");

        handle.join().unwrap();
        let status = gui_status.lock().unwrap();
        status.status.unwrap()
    }
}