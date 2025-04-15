use crate::{
    config,
    pipeline::{Error, Stage},
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

impl Stage for LoadConfigStage {
    fn name(&self) -> &'static str {
        "load_config"
    }

    fn run(&self, ctx: &mut crate::pipeline::Context) -> Result<(), crate::pipeline::Error> {
        let config = config::load_site_config(&self.path)
            .map_err(|e| Error::ConfigLoad(format!("{}", e)))?;

        ctx.config = Some(config);
        Ok(())
    }
}
