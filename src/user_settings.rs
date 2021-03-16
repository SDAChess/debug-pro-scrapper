use confy::ConfyError;
use serde::{Deserialize, Serialize};


#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct UserSettings {
    pub name: String,
    pub surname: String,
    pub login: String,
    pub email: String,
}

pub fn get_user_settings() -> Result<UserSettings, ConfyError> {
    confy::load::<UserSettings>("scrap-practical")
}
