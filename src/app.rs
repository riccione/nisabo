use std::path::{PathBuf};

#[derive(Default)]
pub struct App {
    pub archive_name: String,
    archive_path: Option<PathBuf>
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self {
            archive_name: String::new(),
            archive_path: None,
        };
        // customize egui with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals
        Self::default()
    }
}
