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
    sprites: HashMap<String, (String, usize)>, // (sheet_index, sprite_index)
    sprite_sheets: HashMap<String, SpriteSheet>,
    fallback_sprite: (Texture2D, Rectangle),
}

impl Resources {
    pub fn get_sprite(&self, name: &str) -> (&Texture2D, &Rectangle) {
        if let Some((sheet_name, sprite_index)) = self.sprites.get(name) {
            return self
                .sprite_sheets
                .get(sheet_name)
                .and_then(|sheet| Some((&sheet.texture, sheet.sprites.get(*sprite_index)?)))
                .unwrap_or((&self.fallback_sprite.0, &self.fallback_sprite.1));
        }
        (&self.fallback_sprite.0, &self.fallback_sprite.1)
    }
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

    let files = glob("resources/**/*.png").unwrap();
    let mut sprite_sheets: HashMap<String, SpriteSheet> = HashMap::new();
    for file in files {
        let path = file.unwrap();
        let file_path: String = path.clone().into_os_string().into_string().unwrap();
        let sprite_sheet = load_sprite_sheet(rl, thread, file_path.as_str());
        sprite_sheets.insert(
            path.file_prefix()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap(),
            sprite_sheet,
        );
    }
    let mut sprites = HashMap::new(); // Populate as needed
    for (name, sheet) in &sprite_sheets {
        for idx in sheet.sprites.iter().enumerate() {
            let sprite_name = format!("{}_{}", name, idx.0);
            sprites.insert(sprite_name, (name.clone(), idx.0));
        }
    }

    let fallback_sprite = rl
        .load_texture_from_image(
            thread,
            &Image::gen_image_checked(10, 10, 5, 5, Color::WHITE, Color::BLACK),
        )
        .unwrap();
    let rect = Rectangle::new(0.0, 0.0, 10.0, 10.0);

    Resources {
        fonts: font_map,
        fallback_font,
        sprite_sheets,
        sprites,
        fallback_sprite: (fallback_sprite, rect),
    }
}

// A struct to hold slices from a sprite sheet
pub struct SpriteSheet {
    pub texture: Texture2D,
    pub sprites: Vec<Rectangle>,
}

fn load_sprite_sheet(rl: &mut RaylibHandle, thread: &RaylibThread, path: &str) -> SpriteSheet {
    let texture = rl.load_texture(thread, path).unwrap();
    // Extract dimensions from file name suffix, e.g., "sprites_32x32.png"
    let file_name = std::path::Path::new(path)
        .file_prefix()
        .unwrap()
        .to_str()
        .unwrap();
    let dims_part = file_name.split('_').last().unwrap();
    let mut dims_iter = dims_part.split('x');
    let sprite_width: i32 = dims_iter.next().unwrap().parse().unwrap();
    let sprite_height: i32 = dims_iter.next().unwrap().parse().unwrap();

    let cols = texture.width / sprite_width;
    let rows = texture.height / sprite_height;
    let mut sprites = Vec::with_capacity((cols * rows) as usize);

    for y in 0..rows {
        for x in 0..cols {
            let slice = Rectangle {
                x: (x * sprite_width) as f32,
                y: (y * sprite_height) as f32,
                width: sprite_width as f32,
                height: sprite_height as f32,
            };
            sprites.push(slice);
        }
    }

    SpriteSheet { texture, sprites }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    // TODO: make this lock shared across test files.
    static LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn test_load_config() {
        let config = load_config();
        assert!(config.get_string("app_name").is_ok());
    }

    #[test]
    fn test_load_resources() {
        let _guard = match LOCK.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let (mut rl, thread) = raylib::init().size(640, 480).title("Test").build();
        let resources = load_resources(&mut rl, &thread);
        assert!(resources.fonts.is_empty());
        assert!(resources.fallback_font.is_font_valid());
    }

    #[test]
    fn test_get_sprite() {
        let _guard = match LOCK.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let (mut rl, thread) = raylib::init().size(640, 480).title("Test").build();
        let resources = load_resources(&mut rl, &thread);
        let (texture, rect) = resources.get_sprite("non_existent_sprite");
        assert_eq!(texture.id, resources.fallback_sprite.0.id);
        assert_eq!(*rect, resources.fallback_sprite.1);
    }
}
