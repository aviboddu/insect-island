use std::collections::HashMap;

use config::{Config, File};
use glob::glob;
use raylib::prelude::*;

pub fn load_config() -> Config {
    Config::builder()
        .add_source(
            glob("config/*.json")
                .unwrap()
                .map(|path| File::from(path.unwrap()))
                .collect::<Vec<_>>(),
        )
        .build()
        .expect("Failed to load configuration")
}

pub struct Resources {
    pub fonts: HashMap<String, Font>,
    pub fallback_font: Font,
}

pub fn load_resources(rl: &mut RaylibHandle, thread: &RaylibThread) -> Resources {
    let files = glob("resources/**/*.ttf").unwrap();
    let mut font_map: HashMap<String, Font> = HashMap::new();
    for file in files {
        let path = file.unwrap();
        let file_path: String = path.clone().into_os_string().into_string().unwrap();
        let font = rl.load_font(thread, file_path.as_str()).unwrap();
        let file_name = path
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();
        font_map.insert(file_name, font);
    }
    let fallback_font = unsafe { Font::from_raw(rl.get_font_default().to_raw()) };
    Resources {
        fonts: font_map,
        fallback_font,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let config = load_config();
        // Add assertions based on expected configuration values
        assert!(config.get_string("app_name").is_ok());
    }

    #[test]
    fn test_load_resources() {
        let (mut rl, thread) = raylib::init().size(640, 480).title("Test").build();
        let resources = load_resources(&mut rl, &thread);
        assert!(resources.fonts.is_empty());
        assert!(resources.fallback_font.is_font_valid());
    }
}
