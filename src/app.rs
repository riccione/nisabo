use std::path::{PathBuf};
use rfd::FileDialog;
use eframe::egui;
use log::{info, error};
use crate::config::AppConfig;
use std::error::Error;

#[derive(PartialEq)]
pub enum SidebarTab {
    Notes,
    Trash,
}

#[derive(Default)]
pub struct App {
    pub archive_name: String,
    pub archive_path: Option<PathBuf>,
    pub db_path: String,
    pub selected_file_content: Option<String>,
    pub show_about: bool,
    pub rename_target: Option<PathBuf>,
    pub rename_input: String,
    pub state_rename: bool,
    pub rename_error: Option<String>,
    pub db_error: Option<String>,
    pub load_rows: bool, // trigger loading
    pub names: Vec<(i32, String)>,
    pub notes_deleted: Vec<(i32, String)>,
    pub state_trash_load: bool, // trigger loading
    pub selected_index: Option<i32>,
    pub state_start: bool,
    pub selected_tab: SidebarTab,
    pub show_settings: bool,
    pub font_size: f32,
    pub default_font_size: f32,
    pub config: AppConfig,
}

impl Default for SidebarTab {
    fn default() -> Self {
        SidebarTab::Notes
    }
}

impl App {
    pub fn default_values() -> Self {
        Self {
            archive_name: String::new(),
            archive_path: None,
            db_path: String::new(),
            selected_file_content: None,
            show_about: false,
            rename_target: None,
            rename_input: String::new(),
            state_rename: false,
            rename_error: None,
            db_error: None,
            load_rows: false,
            names: Vec::<(i32, String)>::new(),
            notes_deleted: Vec::<(i32, String)>::new(),
            state_trash_load: false,
            selected_index: None,
            state_start: false,
            selected_tab: SidebarTab::Notes,
            show_settings: false,
            font_size: 13.0,
            default_font_size: 13.0,
            config: AppConfig::load_config(),
        }
    }

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default_values();
        
        let config = AppConfig::load_config();
        if let Some(x) = config.last_archive_path {
            if x.exists() {
                app.archive_path = Some(x.clone());
                app.db_path = x.to_string_lossy().into_owned();
                app.state_start = true;
            }
        }
        
        app.font_size = if config.font_size < 1.0 { // without config.toml file
            app.default_font_size
        } else {
            config.font_size
        };
        app.apply_font_size(&cc.egui_ctx); 

        app
    }

    pub fn create_db(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(path) = FileDialog::new().pick_folder() {
            let archive_name = if self.archive_name.is_empty() {
                "archive"
            } else {
                &self.archive_name
            };

            let archive_path = path.join(format!("{}.db", archive_name));
            
            if archive_path.try_exists()? {
                self.db_error = Some(format!("Database already exists at {:?}", 
                                             archive_path));
            } else {
                if let Some(path_str) = archive_path.to_str() {
                    let db = crate::db::database::Database::new(path_str)?;
                    db.init_tables()?;
                    
                    let config = AppConfig {
                        last_archive_path: Some(archive_path.clone()),
                        font_size: self.font_size,
                    };
                    config.save_config();

                    self.archive_path = Some(archive_path.clone());
                    self.state_start = true;
                    self.db_path = archive_path.to_string_lossy().into_owned();
                } else {
                    self.db_error = Some("Path contains invalid UTF-8".to_string());
                }
            }
        } else {
            error!("No directory selected");
        }
        Ok(())
    }
    
    pub fn open_archive(&mut self) {
        if let Some(path) = FileDialog::new().pick_file() {
            info!("Archive opened from: {}", path.display());
            self.archive_path = Some(path.clone());
            let config = AppConfig {
                last_archive_path: self.archive_path.clone(),
                font_size: self.font_size,
            };
            config.save_config();
            
            let x = path.clone();
            self.db_path = x.to_string_lossy().into_owned();
            self.state_start = true;
        } else {
            error!("No db file selected");
        }
    }
    
    pub fn show_db_ls(&mut self, ui: &mut egui::Ui) 
        -> Result<(), Box<dyn Error>> {
        if !self.load_rows {
            let db = crate::db::database::Database::new(&self.db_path)?;
            match db.get_notes() {
                Ok(names) => {
                    self.names = names;
                    self.load_rows = true; // TODO: move it to state
                }
                Err(e) => {
                    error!("Error loading names from table archive: {e}");
                    self.names.clear();
                }
            }
        }

        if self.names.is_empty() {
            ui.label("No notes found");
        } else {
            egui::ScrollArea::vertical().show(ui, |ui| {
                // for borrow issues
                let xs: Vec<(i32, String)> = self.names.iter()
                    .map(|(id, name)| (*id, name.clone()))
                    .collect();
                for (id, name) in xs {
                    let selected = Some(&id) == self.selected_index.as_ref();

                    let response = ui.add(egui::SelectableLabel::new(selected, &name));

                    if response.clicked() {
                        self.selected_index = Some(id);
                        println!("note clicked {:?}", self.selected_index);
                    }

                    // right btn
                    response.context_menu(|ui| {
                        if ui.button("Rename").clicked() {
                            info!("Rename clicked");
                            self.rename_input = name.to_string();
                            self.selected_index = Some(id);
                            // show popup with name as input
                            self.state_rename = true;
                            ui.close_menu();
                        }

                        if ui.button("Delete").clicked() {
                            info!("Delete clicked");
                            // TODO: handle delete
                            let _ = self.try_delete_note(id);
                            ui.close_menu();
                        }
                    });
                }
            });
        }
        Ok(()) 
    }
    
    pub fn show_trash(&mut self, ui: &mut egui::Ui) 
        -> Result<(), Box<dyn Error>> {
        if !self.state_trash_load {
            let db = crate::db::database::Database::new(&self.db_path)?;
            match db.get_trash() {
                Ok(x) => {
                    self.notes_deleted = x;
                    self.state_trash_load = true; // TODO: move it to state
                }
                Err(e) => {
                    error!("Error loading notes from table note: {e}");
                    self.notes_deleted.clear();
                }
            }
        }

        if self.notes_deleted.is_empty() {
            ui.label("Trash is empty");
        } else {
            egui::ScrollArea::vertical().show(ui, |ui| {
                // for borrow issues
                let xs: Vec<(i32, String)> = self.notes_deleted.iter()
                    .map(|(id, name)| (*id, name.clone()))
                    .collect();
                for (id, name) in xs {
                    let selected = Some(&id) == self.selected_index.as_ref();

                    let response = ui.add(egui::SelectableLabel::new(selected, name));

                    if response.clicked() {
                        self.selected_index = Some(id);
                        println!("note clicked {:?}", self.selected_index);
                    }

                    // right btn
                    response.context_menu(|ui| {
                        ui.set_min_width(120.0);
                        if ui.button("Restore").clicked() {
                            let _ = self.try_restore_note(id);
                            ui.close_menu();
                        }

                        if ui.button("Permanently Delete").clicked() {
                            info!("Delete clicked");
                            let _ = self.try_permanently_delete(id);
                            ui.close_menu();
                        }
                    });
                }
            });
        }
        Ok(()) 
    }


    pub fn show_rename(&mut self, ctx: &egui::Context) { 
        if self.state_rename {
            // tmp var
            let mut open = self.state_rename;
            egui::Window::new("Rename File")
                .open(&mut open)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("Enter new name: ");
                    if let Some(e) = &self.rename_error {
                        ui.label(egui::RichText::new(e).color(egui::Color32::RED));
                    }

                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut self.rename_input);
                        
                        if self.rename_input.trim().is_empty() {
                            self.rename_error = Some("Name cannot be empty".to_string());
                        } else {
                            if ui.button("Rename").clicked() {
                                if let Err(e) = self.try_rename_note() {
                                    error!("Rename failed: {e}");
                                }
                            }
                        }
                  
                        if ui.button("Cancel").clicked() {
                            info!("Cancel clicked");
                            self.state_rename = false;
                            self.rename_target = None;
                            self.rename_input.clear();
                            self.rename_error = None;
                        }
                    });
                });
            if !open {
                self.state_rename = false;
                self.rename_target = None;
                self.rename_input.clear();
                self.rename_error = None;
            }
        }
    }

    fn try_rename_note(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let db = crate::db::database::Database::new(&self.db_path)?;
        println!("{:?}", self.selected_index);
        let _ = db.update_note_name(self.selected_index.unwrap(), &self.rename_input);

        self.state_rename = false;
        self.rename_target = None;
        self.rename_input.clear();
        self.rename_error = None;
        // refresh ui
        self.load_rows = false;
        Ok(())
    }
    
    fn try_delete_note(&mut self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        println!("id: {:?}", id);
        let db = crate::db::database::Database::new(&self.db_path)?;
        let _ = db.delete_note_soft(id);

        // refresh ui
        self.load_rows = false;
        self.state_trash_load = false;
        Ok(())
    }
    
    fn try_restore_note(&mut self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        println!("id: {:?}", id);
        let db = crate::db::database::Database::new(&self.db_path)?;
        let _ = db.restore_note(id);

        // refresh ui
        self.load_rows = false;
        self.state_trash_load = false;
        Ok(())
    }
    
    fn try_permanently_delete(&mut self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        println!("id: {:?}", id);
        let db = crate::db::database::Database::new(&self.db_path)?;
        let _ = db.delete_note_hard(id);

        // refresh ui
        self.load_rows = false;
        self.state_trash_load = false;
        Ok(())
    }

    pub fn show_font_settings(&mut self, ctx: &egui::Context) {
        let mut open = self.show_settings;

        if self.show_settings {
            egui::Window::new("Font Settings")
                .collapsible(false)
                .resizable(false)
                .default_width(250.0)
                .open(&mut open) // toggles based on state
                .show(ctx, |ui| {
                    ui.label("Select font size:");

                    let font_sizes = vec![12.0, 13.0, 14.0, 16.0, 18.0, 20.0, 22.0, 24.0];
                    let mut current_size = self.font_size;

                    egui::ComboBox::from_label("")
                        .selected_text(format!("{:.1}", current_size))
                        .show_ui(ui, |ui| {
                            for &size in &font_sizes {
                                if ui
                                    .selectable_value(&mut current_size, size, format!("{size}"))
                                    .clicked()
                                {
                                    self.font_size = size;
                                    self.config.font_size = size;
                                    self.apply_font_size(ctx);
                                    self.config.save_config();
                                }
                            }
                        });

                    if ui.button("Reset to default").clicked() {
                        self.font_size = self.default_font_size;
                        self.apply_font_size(ctx);
                    }

                    if ui.button("Close").clicked() {
                        self.show_settings = false;
                    }
                });
            if !open {
                self.show_settings = false;
            }
        }
    }
    
    fn apply_font_size(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();

        style.text_styles = [
            (egui::TextStyle::Heading, egui::FontId::new(self.font_size + 6.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Body, egui::FontId::new(self.font_size, egui::FontFamily::Proportional)),
            (egui::TextStyle::Monospace, egui::FontId::new(self.font_size - 2.0, egui::FontFamily::Monospace)),
            (egui::TextStyle::Button, egui::FontId::new(self.font_size, egui::FontFamily::Proportional)),
            (egui::TextStyle::Small, egui::FontId::new(self.font_size - 4.0, egui::FontFamily::Proportional)),
        ]
        .into();

        ctx.set_style(style);
    }
}
