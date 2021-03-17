use crate::user_settings;
use crate::user_settings::UserSettings;
use argh::FromArgs;
use scraper::{Html, Selector};
use std::fs::File;
use std::io::Write;
use std::process::Command;

#[derive(FromArgs, PartialEq, Debug)]
/// Pulls the practical with the url passed as argument.
#[argh(subcommand, name = "pull")]
pub struct PullOptions {
    #[argh(positional)]
    /// the url to pull the practical from
    url: String,
    #[argh(option)]
    /// the directory where to store the pulled file, default is current
    directory: Option<String>,
}

pub fn execute(options: PullOptions) {
    let cfg = match user_settings::get_user_settings() {
        Ok(t) => t,
        Err(t) => {
            eprintln!("{:#}", t);
            std::process::exit(1);
        }
    };
    if cfg == UserSettings::default() {
        println!(
            "Please configure your profile using the \
                 scrap-practical configure\' command!"
        );
        return;
    }

    let resp = match reqwest::blocking::get(options.url) {
        Ok(t) => t,
        Err(t) => {
            eprintln!("{:#}", t);
            std::process::exit(1);
        }
    };

    let body = match resp.text() {
        Ok(t) => t,
        Err(t) => {
            eprintln!("{:#}", t);
            std::process::exit(1);
        }
    };
    let fragment = Html::parse_document(&body);
    // parses based on a CSS selector
    let stories = match Selector::parse("code.language-bash") {
        Ok(t) => t,
        Err(_) => {
            eprintln!("Error while parsing the HTML document");
            std::process::exit(1);
        }
    };
    let elem = match fragment.select(&stories).next() {
        Some(t) => t,
        None => {
            eprintln!("Could not find necessary tags in url!");
            std::process::exit(1);
        }
    };
    let line = match elem.text().next() {
        Some(t) => t,
        None => {
            eprintln!("Could not get the values in the website");
            std::process::exit(1);
        }
    };

    let line = line.trim();
    //We cut the $ git clone from the command to get the arguments
    //This is dirty and will need to be changed...
    let git_url = match line.get(12..) {
        Some(t) => t,
        None => {
            println!("The value that was recovered could not be parsed.");
            std::process::exit(1);
        }
    };

    let git_url = git_url.replace("john.smith", &cfg.login);
    let mut authors_content = cfg.to_vector().join("\n");
    authors_content.push_str("\n");
    let mut git_clone = Command::new("git");
    git_clone.arg("clone").arg(&git_url);
    let mut tp_path;
    if options.directory.is_some() {
        let directory = options.directory.unwrap();
        tp_path = String::from(&directory);
        git_clone.arg(directory);
    } else {
        tp_path = match git_url.get(44..) {
            Some(t) => String::from(t),
            None => {
                println!("The value that was recovered could not be parsed.");
                std::process::exit(1);
            }
        };
    }
    match git_clone.status() {
        Err(t) => {
            eprintln!("{}", t);
            std::process::exit(1);
        }
        _ => (),
    }
    tp_path.push_str("/AUTHORS");
    let mut f = match File::create(&tp_path) {
        Err(t) => {
            eprintln!("{}", t);
            return;
        }
        Ok(f) => f,
    };
    match f.write_all(authors_content.as_bytes()) {
        Ok(_) => println!("Succesfuly wrote AUTHORS file."),
        Err(t) => {
            eprintln!("{}", t);
            std::process::exit(1);
        }
    }
}
