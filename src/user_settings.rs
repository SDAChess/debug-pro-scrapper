use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct UserSettings {
    name: String,
    surname: String,
}

pub fn get_user_settings() -> Option<UserSettings> {
    let cfg: UserSettings = confy::load("debug-pro").expect("Could not load config file");
    if cfg == UserSettings::default() {
        None
    } else {
        Some(cfg)
    }
}
