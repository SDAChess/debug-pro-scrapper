use confy::ConfyError;
use serde::{Deserialize, Serialize};


#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct UserSettings {
    name: String,
    surname: String,
}

pub fn get_user_settings() -> Result<UserSettings, ConfyError> {
    let cfg: Result<UserSettings, ConfyError> = confy::load("debug-pro");
    cfg
}
