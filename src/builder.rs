use crate::{
    pipeline::{self, LoadConfigStage, LoadTemplatesStage, PipelineError},
    writer,
};
use std::{io, path::Path};

// TODO: Read these from rustypages.toml
const OUT_DIR: &str = "dist/";

pub fn build(path: &Path) -> Result<(), PipelineError> {
    fn visit_dir(dir: &Path, content_root: &Path, output_root: &Path) -> io::Result<()> {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                visit_dir(&path, content_root, output_root)?;
            } else if is_markdown_file(&path) {
                let output_path = to_output_path(&path, content_root, output_root)?;

                let markdown = std::fs::read_to_string(&path)?;
                let html = writer::write_html_from_markdown(&markdown);

                if let Some(parent) = output_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }

                std::fs::write(output_path, html)?;
            }
        }
        Ok(())
    }
    /*

    let output_path_buf = path.join(OUT_DIR);
    let output_path = output_path_buf.as_path();
    let content_path_buf = path.join("content");
    let content_path = content_path_buf.as_path();
    visit_dir(content_path, content_path, output_path)
    */

    let mut pipeline = pipeline::Pipeline::new();
    pipeline.add(LoadConfigStage::new("rustypages.toml"));
    pipeline.add(LoadTemplatesStage::new());

    pipeline.run()
}

fn is_markdown_file(path: &Path) -> bool {
    matches!(path.extension().and_then(|ext| ext.to_str()), Some("md" | "mdx"))
}

fn to_output_path(
    file_path: &Path,
    content_root: &Path,
    output_root: &Path,
) -> io::Result<std::path::PathBuf> {
    let relative_path = file_path.strip_prefix(content_root).unwrap();
    let mut new_path = output_root.join(relative_path);
    new_path.set_extension("html");
    Ok(new_path)
}
