//! Config manager for KaGRiS

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer};
use std::fs::{File, OpenOptions};

#[derive(Serialize, Deserialize)]
pub struct Config {
    /// Version of the config
    version: String,
    /// Whether to use Vertical Sync
    pub vsync: bool,
    /// Opacity percentage of the grid (0-100)
    pub grid_opacity: u8,
    /// Volume percentage of the music (0-100)
    pub music_volume: f32,
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
            version: env!("CARGO_PKG_VERSION").to_string(),
            vsync: true,
            grid_opacity: 5,
            music_volume: 50.,
        }
    }
}
