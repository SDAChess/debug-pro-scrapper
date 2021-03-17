use crate::user_settings;
use crate::error::{ScrapperResult, ScrapperError};
use argh::FromArgs;
use scraper::{Html, Selector};
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::path::Path;

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

pub fn execute(options: PullOptions) -> ScrapperResult<()> {
    let cfg = user_settings::get_user_settings()?;
    let resp = reqwest::blocking::get(options.url)?;

    let body = resp.text()?;
    let fragment = Html::parse_document(&body);

    // parses based on a CSS selector
    let stories = Selector::parse("code.language-bash")
        .map_err(|e| ScrapperError::MalformedHtmlError(
            format!("Not valid HTML, parsing error: {:?}", e)
        ))?;

    let elem = fragment
        .select(&stories)
        .next()
        .ok_or(ScrapperError::MalformedHtmlError(
            "Could not find css path 'code.language-bash' in the body".into()
        ))?;

    let line = elem
        .text()
        .next()
        .ok_or(ScrapperError::MalformedHtmlError(
            "The first 'code.language-bash' element has no content".into()
        ))?
        .trim();

    // We cut the $ git clone from the command to get the arguments
    // This is dirty and will need to be changed...
    let git_url = line
        .get(12..)
        .ok_or(ScrapperError::MalformedHtmlError(
            "The first 'code.language-bash' element should have a git URL at 1:12".into()
        ))?
        .replace("john.smith", &cfg.login);

    let authors_content = cfg.to_vector().join("\n") + "\n";
    let mut git_clone = Command::new("git");
    git_clone.arg("clone").arg(&git_url);

    let tp_path = Path::new(if let Some(dir) = &options.directory {
        git_clone.arg(dir);
        dir.as_str()
    } else {
        git_url
            .get(44..)
            .ok_or(ScrapperError::MalformedHtmlError(
                "The first 'code.language-bash' element should have a cloning path at 1:44".into()
            ))?
    });

    git_clone.status().map_err(|e| ScrapperError::GitExecutionError(e))?;

    let authors_path = tp_path.join("AUTHORS");
    let mut f = File::create(&authors_path)
        .map_err(|e| ScrapperError::IoError {
            message: format!("Error while creating file at '{:?}'", &authors_path),
            error: e
        })?;

    f.write_all(authors_content.as_bytes())
        .map_err(|e| ScrapperError::IoError {
            message: format!("Unable to write in file '{:?}'", &authors_path),
            error: e
        })?;

    println!("Successfully wrote AUTHORS file.");

    Ok(())
}
