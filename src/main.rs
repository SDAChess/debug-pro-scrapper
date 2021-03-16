mod user_settings;
mod commands;

use crate::commands::pull::execute;
use crate::user_settings::UserSettings;

use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Top-level command.
struct TopLevel {
    #[argh(subcommand)]
    nested: CommandEnum,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum CommandEnum{
    Configure(commands::configure::ConfigureOptions),
    Pull(commands::pull::PullOptions),
}


fn main() {
    let options: TopLevel = argh::from_env();
    match options.nested {
        CommandEnum::Configure(_) => commands::configure::execute(),
        CommandEnum::Pull(options) => commands::pull::execute(options),
    }
    let cfg = match user_settings::get_user_settings() {
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
