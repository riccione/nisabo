mod app;
mod app_eframe;

use app::App;

const APP_NAME: &str = env!("CARGO_PKG_NAME");

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        APP_NAME,
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc))))
    )
}
