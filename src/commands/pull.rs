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


pub fn execute(options: PullOptions) -> () {
    dbg!(options);
}
