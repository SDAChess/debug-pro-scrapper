use serde::{Deserialize, Serialize};
use crate::error::{ScrapperResult, ScrapperError};

#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct UserSettings {
    pub name: String,
    pub surname: String,
    pub login: String,
    pub email: String,
}

pub fn get_user_settings() -> ScrapperResult<UserSettings> {
    let cfg = confy::load::<UserSettings>("scrap-practical")?;
    if cfg == UserSettings::default() {
        return Err(ScrapperError::NotConfigured);
    }

    Ok(cfg)
}

impl UserSettings {
    // TODO: Change this
    pub fn to_vector(&self) -> Vec<String> {
        let str_vector = vec![
            self.name.clone(),
            self.surname.clone(),
            self.login.clone(),
            self.email.clone(),
        ];
        return str_vector;
    }
}
