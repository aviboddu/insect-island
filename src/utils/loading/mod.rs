use config::{Config, File};
use glob::glob;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let config = load_config();
        // Add assertions based on expected configuration values
        assert!(config.get_string("app_name").is_ok());
    }
}
