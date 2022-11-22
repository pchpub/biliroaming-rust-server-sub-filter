use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub subs: Vec<String>,
    pub auto_mode: bool,
    pub auto_mode_config_paths: Vec<String>,
    // pub biliroaming_rust_server_api: bool,
}