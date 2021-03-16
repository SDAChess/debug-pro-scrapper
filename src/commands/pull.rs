use crate::user_settings;
use crate::user_settings::UserSettings;

use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Pulls the practical with the url passed as argument.
#[argh(subcommand, name = "pull")]
pub struct PullOptions {
    #[argh(positional)]
    /// the url to pull the practical from
    url: String,
    #[argh(option, default = "String::from(\".\")")]
    /// the directory where to store the pulled file, default is current
    directory: String,
}

pub fn execute(options: PullOptions) {
    let cfg = match user_settings::get_user_settings() {
        Ok(t) => t,
        Err(t) => {
            eprintln!("{:#}", t);
            std::process::exit(1);
        }
    };
    dbg!(options.url);
    dbg!(options.directory);
    if cfg == UserSettings::default() {
        println!("Please configure your profile using the configure command!");
        return;
    }

}
