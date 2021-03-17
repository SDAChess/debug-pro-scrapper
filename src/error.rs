use std::fmt::Formatter;

pub type ScrapperResult<T> = Result<T, ScrapperError>;

#[derive(Debug)]
pub enum ScrapperError {
    NotConfigured,

    ConfigurationError {
        error: confy::ConfyError
    },

    HttpError {
        error: reqwest::Error
    },

    MalformedHtmlError(String),
    GitExecutionError(std::io::Error), // TODO: Give command in context

    IoError {
        message: String,
        error: std::io::Error
    }
}

impl std::fmt::Display for ScrapperError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use ScrapperError::*;

        match self {
            NotConfigured => write!(f, "Please configure your profile using the \
                 \'scrap-practical configure\' command!"),
            ConfigurationError { error } => {
                write!(f, "Configuration error, confy dropped: '{}'", error)
            },
            HttpError { error } => {
                write!(f, "HTTP request error, reqwest dropped '{}'", error)
            },
            MalformedHtmlError(message) => {
                write!(f, "Malformed HTML: {}!", message)
            },
            GitExecutionError(error) => {
                write!(f, "Error while executing git: {}!", error)
            },
            IoError { message, error } => {
                write!(f, "I/O error: {}: {}", message, error)
            }
        }
    }
}

impl From<reqwest::Error> for ScrapperError {
    fn from(e: reqwest::Error) -> Self {
        Self::HttpError { error: e }
    }
}

impl From<confy::ConfyError> for ScrapperError {
    fn from(e: confy::ConfyError) -> Self {
        Self::ConfigurationError { error: e }
    }
}
