use crate::user_settings::UserSettings;
use argh::FromArgs;
use std::io;

#[derive(FromArgs, PartialEq, Debug)]
/// Configures the program with the correct values for the user.
#[argh(subcommand, name = "configure")]
pub struct ConfigureOptions {}

fn get_user_input_trimmed(val: &mut String) -> String {
    match io::stdin().read_line(val) {
        Err(t) => {
            eprintln!("{:#}", t);
            std::process::exit(1);
        }
        _ => (),
    };
    val.trim().to_string()
}

pub fn execute() -> () {
    let mut user_settings = UserSettings::default();

    println!("This interactive session will help you configure the software.");

    println!("Please enter your name");
    let mut val = String::new();
    user_settings.name = get_user_input_trimmed(&mut val);

    println!("Please enter your surname");
    let mut val = String::new();
    user_settings.surname = get_user_input_trimmed(&mut val);

    println!("Please enter your login");
    let mut val = String::new();
    user_settings.login = get_user_input_trimmed(&mut val);

    println!("Please enter your email");
    let mut val = String::new();
    user_settings.email = get_user_input_trimmed(&mut val);

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
