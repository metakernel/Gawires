use std::{collections::HashMap, path::PathBuf};
use dirs::home_dir;

/// TODO - Put this in UserConfig.rs instead of here

/// The user configuration file path.

/// Data structure for the user's configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserConfig{
    /// Path to the user configuration file.
    config_file_path: Option<std::path::PathBuf>,
    /// Dictionnary of the user's configuration as Config Name => Value.
    configs: HashMap<String, String>,
}

impl UserConfig{
    /// Creates a new user configuration.
    pub fn new() -> UserConfig{
        UserConfig{
            config_file_path: None,
            configs: HashMap::new(),
        }
    }
    /// Sets the user configuration file path.
    pub fn set_config_file_path(&mut self, path: std::path::PathBuf){
        self.config_file_path = Some(path);
    }
    /// Gets the user configuration file path.
    pub fn get_config_file_path(&self) -> Option<std::path::PathBuf>{
        self.config_file_path.clone()
    }
    /// Sets the user configuration.
    pub fn set_config(&mut self, config: String, value: String){
        self.configs.insert(config, value);
    }
    /// Gets the user configuration.
    pub fn get_config(&self, config: String) -> Option<String>{
        self.configs.get(&config).map(|x| x.clone())
    }

    /// Get the configuration root path.
    pub fn get_gawires_configs_root() -> PathBuf {
        home_dir().unwrap().join(".gaw/")
    } 

    // Get the user configuration default path.
    pub fn get_def_user_config_path() -> PathBuf {
        Self::get_gawires_configs_root().join("localuser.conf")
    }
}
