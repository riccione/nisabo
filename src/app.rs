use eframe::egui;

#[derive(Default)]
pub struct App {}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // customize egui with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals
        Self::default()
    }
}
