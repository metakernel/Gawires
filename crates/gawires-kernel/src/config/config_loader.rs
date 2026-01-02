// TODO: Create a config loader trait

use super::user_config::{get_default_user_config_path, UserConfig};
use std::io::{BufReader, BufRead};
use std::fs::File;


pub trait ConfigLoader {
    fn load_config(&mut self) -> Result<(), String>;
}

pub struct UserConfigLoader {
    user_config: UserConfig,
}

impl UserConfigLoader {
    pub fn new() -> UserConfigLoader {
        UserConfigLoader {
            user_config: UserConfig::new(),
        }
    }
}

impl ConfigLoader for UserConfigLoader {
    fn load_config(&mut self) -> Result<(), String> {
        let config_path = get_default_user_config_path();
        let config_file = File::open(config_path);
        match config_file {
            Ok(file) => {
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    let line = line.unwrap();
                    let parts: Vec<&str> = line.split("=").collect();
                    if parts.len() == 2 {
                        self.user_config.set_config(parts[0].to_string(), parts[1].to_string());
                    }
                }
                Ok(())
            }
            Err(e) => Err(format!("Error opening file: {}", e)),
        }
    }
}