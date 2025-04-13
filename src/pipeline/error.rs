#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    FrontMatter(String),
    Render(String),
    StageFailed { stage: String, message: String },
    Other(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::FrontMatter(msg) => write!(f, "Frontmatter error: {}", msg),
            Error::Render(msg) => write!(f, "Render error: {}", msg),
            Error::StageFailed { stage, message } => {
                write!(f, "Stage '{}' failed: {}", stage, message)
            }
            Error::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}
