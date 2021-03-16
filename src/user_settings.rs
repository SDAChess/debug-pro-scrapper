use serde::{Serialize, Deserialize};

#[derive(Default,Serialize, Deserialize, Debug)]
pub struct UserSettings {
    name: String,
    surname: String,
}


pub fn get_user_settings() -> UserSettings {
    let cfg: UserSettings = confy::load("debug-pro").expect("Could not load config file");
    return cfg;
}
