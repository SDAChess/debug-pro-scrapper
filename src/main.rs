mod user_settings;
use crate::user_settings::UserSettings;


fn main() {
    let cfg: UserSettings = match user_settings::get_user_settings() {
        Ok(t) => t,
        Err(t) => {
            eprintln!("{:#}", t);
            std::process::exit(1);
        }
    };
    if cfg == UserSettings::default() {
        println!("Please configure your profile!")
    }
    dbg!(cfg);
}
