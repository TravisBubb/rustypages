pub mod context;
pub mod error;
pub mod stage;
pub mod stages;

pub use context::PipelineContext;
pub use error::PipelineError;
pub use stage::PipelineStage;
pub use stages::load_config::LoadConfigStage;
pub use stages::load_templates::LoadTemplatesStage;

/// Represents a series of stages that can be executed
pub struct Pipeline {
    ctx: PipelineContext,
    stages: Vec<Box<dyn PipelineStage>>,
}

impl Pipeline {
    /// Creates a new, empty Pipeline
    pub fn new() -> Self {
        Pipeline { ctx: PipelineContext::new(), stages: Vec::new() }
    }

    /// Adds the provided stage to the pipeline
    pub fn add<T: PipelineStage + 'static>(&mut self, stage: T) {
        self.stages.push(Box::new(stage));
    }

    /// Executes the pipeline
    pub fn run(&mut self) -> Result<(), PipelineError> {
        println!("Beginning pipeline execution...");
        for stage in &self.stages {
            println!("Running stage [{}]", stage.name());
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
