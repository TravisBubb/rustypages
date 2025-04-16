#[derive(Debug)]
pub enum PipelineError {
    Io(std::io::Error),
    ConfigLoad(String),
    FrontMatter(String),
    Render(String),
    StageFailed { stage: String, message: String },
    Other(String),
}

impl std::fmt::Display for PipelineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipelineError::Io(e) => write!(f, "IO error: {}", e),
            PipelineError::ConfigLoad(msg) => write!(f, "Config load error: {}", msg),
            PipelineError::FrontMatter(msg) => write!(f, "Frontmatter error: {}", msg),
            PipelineError::Render(msg) => write!(f, "Render error: {}", msg),
            PipelineError::StageFailed { stage, message } => {
                write!(f, "Stage '{}' failed: {}", stage, message)
            }
            PipelineError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for PipelineError {}

impl From<std::io::Error> for PipelineError {
    fn from(err: std::io::Error) -> Self {
        PipelineError::Io(err)
    }
}
