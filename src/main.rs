mod user_settings;
mod commands;

use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// The toplevel commands
struct TopLevel {
    #[argh(subcommand)]
    commands: CommandEnum,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum CommandEnum {
    Configure(commands::configure::ConfigureOptions),
    Pull(commands::pull::PullOptions),
}

fn main() {
    let options: TopLevel = argh::from_env();

    match options.commands {
        CommandEnum::Configure(_) => commands::configure::execute(),
        CommandEnum::Pull(options) => commands::pull::execute(options),
    }
}
