

use crate::user_settings::get_user_settings;
use crate::user_settings::UserSettings;

mod user_settings;


fn main() {
   let cfg: UserSettings = get_user_settings();
    dbg!(cfg);
}

