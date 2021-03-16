mod user_settings;
use crate::user_settings::UserSettings;


fn main() {
    let cfg: UserSettings = match user_settings::get_user_settings() {
        Some(cfg) => cfg,
        None => panic!("No default configuration! Please configure"),
    };
    dbg!(cfg);
}
