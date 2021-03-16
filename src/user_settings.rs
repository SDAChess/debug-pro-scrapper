use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct UserSettings {
    name: String,
    surname: String,
}

pub fn get_user_settings() -> Option<UserSettings> {
    let cfg: UserSettings = match confy::load("debug-pro") {
        Ok(cfg) => cfg,
        Err(_) => {
            eprintln!("Error while reading the configuration file!");
            std::process::exit(1);
        }
    };
    if cfg == UserSettings::default() {
        None
    } else {
        Some(cfg)
    }
}
