pub mod context;
pub mod error;
pub mod stage;
pub mod stages;

pub use context::Context;
pub use error::Error;
pub use stage::Stage;
pub use stages::load_config::LoadConfigStage;

/// Represents a series of stages that can be executed
pub struct Pipeline {
    ctx: Context,
    stages: Vec<Box<dyn Stage>>,
}

impl Pipeline {
    /// Creates a new, empty Pipeline
    pub fn new() -> Self {
        Pipeline { ctx: Context::new(), stages: Vec::new() }
    }

    /// Adds the provided stage to the pipeline
    pub fn add<T: Stage + 'static>(&mut self, stage: T) {
        self.stages.push(Box::new(stage));
    }

    /// Executes the pipeline
    pub fn run(&mut self) -> Result<(), Error> {
        println!("Beginning pipeline execution...");
        for stage in &self.stages {
            println!("Running stage [{}]", stage.name());
            stage.run(&mut self.ctx)?;
        }

        println!("Context Config: {:?}", self.ctx.config_or_panic());

        Ok(())
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Pipeline::new()
    }
}
