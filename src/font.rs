use std::collections::HashMap;

#[derive(Default)]
pub struct FontManager {
    font_cache: HashMap<String, Vec<u8>>,
}

impl FontManager {
    pub fn new() -> Self {
        Self {
            font_cache: HashMap::new(),
        }
    }

    pub fn get_font(&mut self, name: &str) -> Option<&Vec<u8>> {
        if !self.font_cache.contains_key(name) {
            let path = format!("assets/fonts/{}.ttf", name);
            match std::fs::read(&path) {
                Ok(bytes) => {
                    self.font_cache.insert(name.to_string(), bytes);
                }
                Err(e) => {
                    eprintln!("Failed to load font {}: {}", name, e);
                    return None;
                }
            }
        }
        self.font_cache.get(name)
    }
}
