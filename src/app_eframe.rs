use eframe::egui::{self, Button, Color32, Key, RichText};
use log::{info};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use crate::ui::about::show_about;
use crate::app::{App};

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // apply theme
        ctx.set_visuals(if self.state_is_dark_mode {
            // println!("dark mode is on");
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
      
        // io: export && import
        if let Some(rx) = &self.io_rx {
            if let Ok(progress) = rx.try_recv() {
                self.state_io_progress = Some(progress);
                if progress >= 1.0 {
                    self.state_importing = false;
                    self.state_exporting = false;
                    self.io_rx = None;
                }
            }
        }

        if self.state_exporting {
            self.show_progress_window(
                ctx,
                "Exporting notes..",
                "Export in progress. Please wait..",
                self.state_io_progress,
            );
        }
        
        if self.state_importing {
            self.show_progress_window(
                ctx,
                "Importing notes..",
                "Import in progress. Please wait..",
                self.state_io_progress,
            );
        }

        if self.import_done.load(Ordering::Relaxed) {
            self.import_done.store(false, Ordering::Relaxed); // reset
            self.load_rows = false; // trigger reload
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

            // keyboard shortcuts: 
            // Ctrl+F - search
            // Ctrl+S - save
            ctx.input(|i| {
                if i.key_pressed(Key::F) && i.modifiers.ctrl {
                    self.state_search = true;
                    self.search_has_focus = true;
                } else if i.key_pressed(Key::S) && i.modifiers.ctrl {
                    let _ = self.try_update_note_content();
                }
            });

            if self.state_search {
                self.show_search(ctx);
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

    // auto save on exit
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        if self.edited_content != self.original_content {
            let _ = self.try_auto_update_note_content();
        }
    }
}
