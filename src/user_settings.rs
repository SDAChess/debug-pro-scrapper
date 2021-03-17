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

impl UserSettings {
    pub fn to_vector(&self) -> Vec<String> {
        let str_vector = vec![
            self.name.to_string(),
            self.surname.to_string(),
            self.login.to_string(),
            self.email.to_string(),
        ];
        return str_vector;
    }
}
