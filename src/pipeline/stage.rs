use crate::pipeline::{PipelineContext, PipelineError};

/// Represents a single stage in the pipeline
pub trait PipelineStage {
    /// Returns the name of the pipeline stage
    fn name(&self) -> &'static str;

    /// Executes the pipeline stage
    fn run(&self, ctx: &mut PipelineContext) -> Result<(), PipelineError>;
}
