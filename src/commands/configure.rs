use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Configures the program with the correct values for the user.
#[argh(subcommand, name = "configure")]
pub struct ConfigureOptions{}


pub fn execute() -> () {

}
