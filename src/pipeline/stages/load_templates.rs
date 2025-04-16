use crate::pipeline::PipelineStage;

/// Represents the pipeline stage to load page templates
pub struct LoadTemplatesStage {}

impl LoadTemplatesStage {
    pub fn new() -> Self {
        Self {}
    }
}

impl PipelineStage for LoadTemplatesStage {
    fn name(&self) -> &'static str {
        "load_templates"
    }

    fn run(
        &self,
        ctx: &mut crate::pipeline::PipelineContext,
    ) -> Result<(), crate::pipeline::PipelineError> {
        let template_dir = ctx.config_or_panic().template_dir.clone();
        ctx.templates.load_from_dir(&template_dir)?;

        Ok(())
    }
}

impl Default for LoadTemplatesStage {
    fn default() -> Self {
        LoadTemplatesStage::new()
    }
}
