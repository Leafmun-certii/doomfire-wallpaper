use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub screen_width: Option<usize>,
    pub screen_height: Option<usize>,
    pub scale: Option<usize>,
    pub fps: Option<u32>,
    pub fire_type: Option<String>,
    pub background: Option<[u8; 3]>,
    pub restart_on_pause: Option<bool>,
    pub pause_on_cover: Option<bool>,
    pub screen_burn: Option<bool>,
    pub wind_strength: Option<f64>,
}

impl Config {
    pub fn load() -> Self {
        let config_path = dirs::home_dir()
            .expect("Could not find home directory")
            .join(".config/doomfire-wallpaper/config.toml");
        let config_str = std::fs::read_to_string(config_path).unwrap_or_default();
        let config: Self = toml::from_str(&config_str).unwrap_or_default();
        let default = Config::default();
        Config {
            screen_width: config.screen_width.or(default.screen_width),
            screen_height: config.screen_height.or(default.screen_height),
            scale: config.scale.or(default.scale),
            fps: config.fps.or(default.fps),
            fire_type: config.fire_type.or(default.fire_type),
            background: config.background.or(default.background),
            restart_on_pause: config.restart_on_pause.or(default.restart_on_pause),
            pause_on_cover: config.pause_on_cover.or(default.pause_on_cover),
            screen_burn: config.screen_burn.or(default.screen_burn),
            wind_strength: config.wind_strength.or(default.wind_strength),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            screen_width: Some(1920),
            screen_height: Some(1080),
            scale: Some(4),
            fps: Some(23),
            fire_type: Some("Original".to_string()),
            background: None,
            restart_on_pause: Some(true),
            pause_on_cover: Some(true),
            screen_burn: Some(false), // Default: disabled
            wind_strength: Some(0.5),
        }
    }
}
