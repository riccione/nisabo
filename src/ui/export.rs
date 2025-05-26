use eframe::egui;
use rfd::FileDialog;
use std::error::Error;
use log::{info, error};
use crate::app::{App};

impl App {
    pub fn export(&mut self, target: u8) -> Result<(), Box<dyn Error>> {
        if let Some(path) = FileDialog::new()
            .pick_folder() {

                // 1. read all content from db
                // 2. iterate over records and create md or html files in the selected dir

                println!("{:?}", path);
        } else {
            error!("No directory selected");
        }
        Ok(())
    }
}
