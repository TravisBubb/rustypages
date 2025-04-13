pub mod context;
pub mod error;
pub mod stage;

pub use context::Context;
pub use error::Error;
pub use stage::Stage;

/// Represents a series of stages that can be executed
pub struct Pipeline {
    ctx: Context,
    stages: Vec<Box<dyn Stage>>,
}

impl Pipeline {
    /// Creates a new, empty Pipeline
    pub fn new() -> Self {
        Pipeline { ctx: Context {}, stages: Vec::new() }
    }

    /// Adds the provided stage to the pipeline
    pub fn add<T: Stage + 'static>(&mut self, stage: T) {
        self.stages.push(Box::new(stage));
    }

    /// Executes the pipeline
    pub fn run(&mut self) -> Result<(), Error> {
        for stage in &self.stages {
            stage.run(&mut self.ctx)?;
        }
        Ok(())
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Pipeline::new()
    }
}
