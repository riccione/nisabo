use std::collections::HashMap;
use crate::constants::{DEFAULT_FONT_DIR, DEFAULT_FONT};

#[derive(Default, Clone)]
pub struct FontManager {
    pub current_font: String,
    pub font_dir: String,
    pub fonts: Vec<String>,
    pub font_cache: HashMap<String, Vec<u8>>,
}

impl FontManager {
    
    pub fn new(font_dir: String) -> Self {
        // load DejaVuSans at compile time
        let mut font_cache = HashMap::new();
        let font_name = "DejaVuSans";
        font_cache.insert(font_name.to_string(),
            include_bytes!("../assets/fonts/DejaVuSans.ttf").to_vec());
       
        let fonts = vec![
            DEFAULT_FONT.to_string(), // fallback
            font_name.to_string()
        ];

        let mut fm = Self {
            current_font: DEFAULT_FONT.to_string(),
            font_dir, //DEFAULT_FONT_DIR.to_string(),
            fonts: fonts,
            font_cache 
        };

        fm.load_available_fonts();
        fm
    }

    pub fn load_available_fonts(&mut self) {
        // let font_dir = "assets/fonts";
        println!("load_available fonts {}",self.font_dir);
        // clear fonts before loading
        self.fonts.clear();
        self.fonts.push(DEFAULT_FONT.to_string());

        if let Ok(enteries) = std::fs::read_dir(&self.font_dir) {
            for entry in enteries.flatten() {
                let path = entry.path();

                if path.extension().map_or(false, |ext| ext == "ttf") {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        let font_name = stem.to_string();

                        // skip if font is already in self.fonts
                        if self.fonts.contains(&font_name) {
                            continue;
                        }

                        self.fonts.push(font_name);
                    }
                }
            }
        } else {
            eprintln!("Failed to read dir: {}", self.font_dir);
        }
    }

    pub fn get_font(&mut self, name: &str) -> Option<&Vec<u8>> {
        if !self.font_cache.contains_key(name) {
            // avoid loading DejaVuSans again
            if name == "DejaVuSans" {
                return self.font_cache.get(name);
            }
            
            let path = format!("{}/{}.ttf", 
                               self.font_dir,
                               name);
            println!("full path to the font: {path}");
            match std::fs::read(&path) {
                Ok(bytes) => {
                    self.font_cache.insert(name.to_string(), bytes);
                    self.fonts.push(name.to_string());
                }
                Err(e) => {
                    eprintln!("Failed to load font {}: {}", name, e);
                    return None;
                }
            }
        }
        self.font_cache.get(name)
    }

    pub fn list_fonts(&self) -> &Vec<String> {
        &self.fonts
    }
}
