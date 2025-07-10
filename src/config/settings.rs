use clap::Parser;
use figment::providers::{Env, Format, Serialized, Toml};
use figment::Figment;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Parser, Deserialize, Serialize, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(skip)]
    pub height: f32,
    #[arg(skip)]
    pub width: f32,
    #[arg(short, long)]
    current_directory: Option<PathBuf>,
    #[arg(short, long, default_value = "Mightty")]
    pub window_title: String,
    #[arg(long = "vsync", default_value = "true")]
    pub enable_vsync: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            height: 600.0,
            width: 800.0,
            current_directory: None,
            window_title: "Mightty".to_string(),
            enable_vsync: true,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = config_path();

        Figment::new()
            .merge(Serialized::defaults(Config::default()))
            .merge(Toml::file(config_path))
            .merge(Env::prefixed("MIGHTTY_"))
            .merge(Serialized::defaults(Config::parse()))
            .extract()
            .expect("Failed to extract configuration")
    }
}

fn config_path() -> PathBuf {
    xdg::BaseDirectories::with_prefix("mightty")
        .get_config_file("config.toml")
        .expect("Failed to get config file path")
}
