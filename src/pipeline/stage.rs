use crate::pipeline::{Context, Error};

/// Represents a single stage in the pipeline
pub trait Stage {
    /// Returns the name of the pipeline stage
    fn name(&self) -> &'static str;

    /// Executes the pipeline stage
    fn run(&self, ctx: &mut Context) -> Result<(), Error>;
}
