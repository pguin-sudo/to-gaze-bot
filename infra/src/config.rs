use std::env;

pub struct Config {
    pub telegram: TelegramConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        Config {
            telegram: TelegramConfig::new(),
        }
    }
}

pub struct TelegramConfig {
    pub token: String,
}

impl Default for TelegramConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl TelegramConfig {
    pub fn new() -> Self {
        TelegramConfig {
            token: env::var("TG_TOKEN").unwrap(),
        }
    }
}
