use std::sync::Mutex;
use std::{collections::HashMap, fs};
use toml::Value;

lazy_static::lazy_static! {
    pub static ref CHARCTER_CONFIG: Mutex<Config> = Mutex::new(Config::new());
}

#[derive(Debug)]
pub struct CharacterEntry {
    pub default: String,
    pub id_color: HashMap<i32, String>,
}

#[derive(Debug)]
pub struct Config {
    pub entries: HashMap<String, CharacterEntry>,
}

impl CharacterEntry {
    pub fn new() -> Self {
        Self {
            default: "".to_string(),
            id_color: HashMap::new(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
}

pub fn read_config_file() {
    match fs::read_to_string("rom:/VictoryStage/config.toml") {
        Ok(res) => setup_config(res),
        Err(_) => {
            println!("[Unique Character Result Screen::read_config_file] Failed to read rom:/VictoryStage/info.toml")
        }
    }
}

fn setup_config(content: String) {
    let out = content.parse::<Value>().unwrap();

    for item in out.as_table() {
        for (key, value) in item {

            if !CHARCTER_CONFIG.lock().unwrap().entries.contains_key(key) {
                CHARCTER_CONFIG
                    .lock()
                    .unwrap()
                    .entries
                    .insert(key.to_string(), CharacterEntry::new());
            }

            for val in value.as_table() {
                for (k, v) in val {
                    let v = v.to_string().replace("\"", "");

                    if k == "default" {
                        CHARCTER_CONFIG
                            .lock()
                            .unwrap()
                            .entries
                            .get_mut(key)
                            .unwrap()
                            .default = v.to_string();
                        continue;
                    }

                    CHARCTER_CONFIG
                        .lock()
                        .unwrap()
                        .entries
                        .get_mut(key)
                        .unwrap()
                        .id_color
                        .insert(
                            k[1..].parse::<i32>().unwrap(),
                            v,
                        );
                }
            }
        }
    }
}
