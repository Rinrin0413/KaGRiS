//! Config manager for KaGRiS

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer};
use std::fs::{File, OpenOptions};

#[derive(Serialize, Deserialize)]
pub struct Config {
    /// Version of the config
    version: ConfVer,
    /// Whether to use Vertical Sync
    pub vsync: bool,
    /// Opacity percentage of the grid (0-100)
    pub grid_opacity: u8,
    /// Volume percentage of the music (0-100)
    pub music_volume: f32,
    /// Window mode
    pub window_mode: WindowModeForConf,
}

#[derive(Serialize, Deserialize)]
pub enum ConfVer {
    Invalid,
    #[serde(rename = "v0.1.0")]
    V0_1_0,
}

impl ConfVer {
    /// Get the `ConfVer` from a string.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        match s {
            "v0.1.0" => ConfVer::V0_1_0,
            _ => {
                error!(
                    "Unknown config version: {}\nThis client version is `{}` so config file will be initialized.", 
                    s,
                    Self::current_version()
                );
                Self::Invalid
            }
        }
    }

    /// Get the current version of the config.
    pub fn current_version() -> Self {
        Self::from_str(env!("CARGO_PKG_VERSION"))
    }
}

impl std::fmt::Display for ConfVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfVer::V0_1_0 => write!(f, "v0.1.0"),
            _ => write!(f, "Invalid"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum WindowModeForConf {
    Windowed,
    Fullscreen,
    BorderlessFullscreen,
    SizedFullscreen,
}

impl WindowModeForConf {
    pub fn from_window_mode(window_mode: WindowMode) -> Self {
        match window_mode {
            WindowMode::Windowed => WindowModeForConf::Windowed,
            WindowMode::Fullscreen => WindowModeForConf::Fullscreen,
            WindowMode::BorderlessFullscreen => WindowModeForConf::BorderlessFullscreen,
            WindowMode::SizedFullscreen => WindowModeForConf::SizedFullscreen,
        }
    }

    pub fn to_window_mode(&self) -> WindowMode {
        match self {
            WindowModeForConf::Windowed => WindowMode::Windowed,
            WindowModeForConf::Fullscreen => WindowMode::Fullscreen,
            WindowModeForConf::BorderlessFullscreen => WindowMode::BorderlessFullscreen,
            WindowModeForConf::SizedFullscreen => WindowMode::SizedFullscreen,
        }
    }
}

impl Config {
    /// Load from config.json
    pub fn load() -> Self {
        match File::open("config.json") {
            Ok(f) => {
                match from_reader(f) {
                    Ok(config) => {
                        // info!("Loaded config.json");
                        config
                    }
                    Err(why) => {
                        warn!("Failed to parse config.json: {}", why);
                        default() // TODO: impl ask to initialize
                    }
                }
            }
            Err(why) => {
                warn!("Failed to open config.json: {}", why);
                default() // TODO: impl ask to initialize
            }
        }
    }

    /// Apply to config.json
    pub fn save(&self) {
        match OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("config.json")
        {
            Ok(mut f) => {
                if let Err(why) = to_writer(&mut f, self) {
                    warn!("Failed to save config.json: {}", why);
                }
            }
            Err(why) => {
                warn!("Failed to open config.json: {}", why)
                // TODO: impl ask to initialize
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: ConfVer::from_str(env!("CARGO_PKG_VERSION")),
            vsync: true,
            grid_opacity: 5,
            music_volume: 50.,
            window_mode: WindowModeForConf::Windowed,
        }
    }
}
