//! Handles all configuration related stuff.

use gtk::{traits::GtkWindowExt, ApplicationWindow};
use once_cell::sync::Lazy;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Write;
use std::sync::RwLock;

// Global configuration object with read write lock
pub static CONFIG_LOCK: Lazy<RwLock<Config>> = Lazy::new(|| {
    return RwLock::new(DEFAULT_CONFIG);
});

/// The default config that'll be used if a config file was not found
pub const DEFAULT_CONFIG: Config = Config {
    anchor: WindowAnchor::TopRight,
    margin_x: 10,
    margin_y: 40,

    update_interval_cpu: 1,
    update_interval_disk: 10,
    update_interval_general: 60,
    update_interval_memory: 1,
    update_interval_process: 1,
};

#[derive(Serialize, Deserialize, Debug)]
pub enum WindowAnchor {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub anchor: WindowAnchor,
    pub margin_x: i32,
    pub margin_y: i32,

    pub update_interval_cpu: u32,
    pub update_interval_disk: u32,
    pub update_interval_general: u32,
    pub update_interval_memory: u32,
    pub update_interval_process: u32,
}

impl Config {
    /// Resolves config file path.
    /// Refer to https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html
    /// for more information about config path resolution.
    pub fn get_config_path() -> Option<String> {
        if let Ok(config_dir) = env::var("XDG_CONFIG_HOME") {
            return Some(format!("{}/pomky/config.json", config_dir));
        } else {
            if let Ok(home_dir) = env::var("HOME") {
                return Some(format!("{}/.config/pomky/config.json", home_dir));
            } else {
                return None;
            }
        };
    }

    /// Loads configuration form the config file
    pub fn load() -> Result<(), String> {
        // resolve config file path
        if let Some(config_path) = Config::get_config_path() {
            // try to load config file as a json file
            if let Ok(loaded_config) = serde_json::from_str::<Config>(config_path.as_str()) {
                *(CONFIG_LOCK.write().unwrap()) = loaded_config;
                return Ok(());
            } else {
                // Create default configuration file if it doesn't exist already
                if let Err(err) = DEFAULT_CONFIG.save() {
                    return Err(format!(
                        "Failed to create default configuration file.\n{}",
                        err
                    ));
                }

                // using default config
                return Ok(());
            }
        } else {
            return Err("Failed to locate config file".into());
        }
    }

    /// Saves current configuration to a file
    pub fn save(self: &Self) -> Result<(), String> {
        // serialize current config
        if let Ok(serialized_config) = serde_json::to_string_pretty::<Self>(self) {
            // resolve config file path
            if let Some(config_path) = Config::get_config_path() {
                // create parent directories
                std::fs::create_dir_all(std::path::Path::new(&config_path).parent().unwrap())
                    .unwrap();

                if let Ok(mut file) = File::create(config_path) {
                    // write to file
                    if let Err(err) = file.write_all(serialized_config.as_bytes()) {
                        return Err(format!("Failed to save configuration file.\n{}", err));
                    } else {
                        return Ok(());
                    }
                } else {
                    return Err("Failed to create configuration file".into());
                }
            } else {
                return Err("Failed to locate config file".into());
            }
        } else {
            return Err("Failed to serialize configuration data".into());
        }
    }

    pub fn calculate_position(self: &Self, window: &ApplicationWindow) -> (i32, i32) {
        let window_width = window.default_width();
        let window_height = window.default_height();

        let screen_width;
        let screen_height;
        unsafe {
            screen_width = gdk::ffi::gdk_screen_width();
            screen_height = gdk::ffi::gdk_screen_height();
        }

        return match self.anchor {
            WindowAnchor::TopLeft => (self.margin_x, self.margin_y),
            WindowAnchor::TopRight => (screen_width - window_width - self.margin_x, self.margin_y),
            WindowAnchor::BottomRight => (
                screen_width - window_width - self.margin_x,
                screen_height - window_height - self.margin_y,
            ),
            WindowAnchor::BottomLeft => {
                (self.margin_x, screen_height - window_height - self.margin_y)
            }
        };
    }
}
