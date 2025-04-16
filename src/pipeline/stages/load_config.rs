use crate::{
    config,
    pipeline::{PipelineError, PipelineStage},
};

/// Represents the pipeline stage to load site configuration
pub struct LoadConfigStage {
    path: String,
}

impl LoadConfigStage {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }
}

impl PipelineStage for LoadConfigStage {
    fn name(&self) -> &'static str {
        "load_config"
    }

    fn run(
        &self,
        ctx: &mut crate::pipeline::PipelineContext,
    ) -> Result<(), crate::pipeline::PipelineError> {
        let config = config::load_site_config(&self.path)
            .map_err(|e| PipelineError::ConfigLoad(format!("{}", e)))?;

        ctx.config = Some(config);
        Ok(())
    }
}
