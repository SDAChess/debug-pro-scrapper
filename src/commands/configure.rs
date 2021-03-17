use std::io;
use argh::FromArgs;

use crate::user_settings::UserSettings;

#[derive(FromArgs, PartialEq, Debug)]
/// Configures the program with the correct values for the user.
#[argh(subcommand, name = "configure")]
pub struct ConfigureOptions {}

fn get_user_input_trimmed() -> String {
    let mut val = String::new();
    if let Err(t) = io::stdin().read_line(&mut val) {
        eprintln!("{:#}", t);
        std::process::exit(1);
    };

    val.trim().to_string()
}

pub fn execute() -> () {
    let mut user_settings = UserSettings::default();

    println!("This interactive session will help you configure the software.");

    println!("Please enter your name");
    user_settings.name = get_user_input_trimmed();

    println!("Please enter your surname");
    user_settings.surname = get_user_input_trimmed();

    println!("Please enter your login");
    user_settings.login = get_user_input_trimmed();

    println!("Please enter your email");
    user_settings.email = get_user_input_trimmed();

    println!(
        "The new configuration is :\n
            Name : {}\n
            Surname : {}\n
            Login : {}\n
            Email : {}\n",
        user_settings.name,
        user_settings.surname,
        user_settings.login,
        user_settings.email
    );
    confy::store("scrap-practical", user_settings).expect("Aled!");
}
