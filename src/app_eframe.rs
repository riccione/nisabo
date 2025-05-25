use eframe::egui::{self, Button, Color32, RichText};
use log::{info};
use crate::ui::about::show_about;
use crate::app::{App};

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // apply theme
        ctx.set_visuals(if self.state_is_dark_mode {
            println!("dark mode is on");
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        });

        if self.state_rename {
            self.show_rename(ctx);
        }
        
        if self.show_settings {
            self.show_font_settings(ctx);
        }
        
        if self.state_add_new_note {
            self.show_add_new_note(ctx);
        }

        if self.state_start {
            self.show_menubar(ctx);    
            // must be before sidepanels to reserve the space
            self.show_statusbar(ctx);    

            self.show_toolbar(ctx);

            self.show_sidepanels(ctx);
            
            if self.show_about {
                show_about(ctx, &mut self.show_about);
            }
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);

                    let softer_red = Color32::from_rgb(200, 50, 50);
                    let txt_create_archive = RichText::new("Create Archive")
                        .size(24.0)
                        .color(Color32::WHITE);
                    let btn_create_archive = Button::new(txt_create_archive)
                        .fill(softer_red);

                    if ui.add(btn_create_archive).clicked() {
                        info!("Create Archive clicked");
                        let _ = self.create_db();
                    }
                    
                    info!("{:?}", self.db_error);
                    if let Some(ref err) = self.db_error {
                        ui.colored_label(egui::Color32::RED, format!("{}", err));
                    }

                    ui.add_space(20.0);

                    if ui.button("Open Archive").clicked() {
                        info!("Open Archive clicked");
                        self.open_archive();
                    }
                });
            });
        }
    }
}
