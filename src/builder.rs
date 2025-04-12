use crate::writer;
use std::{io, path::Path};

// TODO: Read these from rustypages.toml
const OUT_DIR: &str = "dist/";
const CONTENT_DIR: &str = "content/";

pub fn build() -> io::Result<()> {
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

    let content_path = Path::new(CONTENT_DIR);
    let output_path = Path::new(OUT_DIR);
    visit_dir(content_path, content_path, output_path)
}

fn is_markdown_file(path: &Path) -> bool {
    matches!(path.extension().and_then(|ext| ext.to_str()), Some("md" | "mdx"))
}

fn to_output_path(
    file_path: &Path,
    content_root: &Path,
    output_root: &Path,
) -> io::Result<std::path::PathBuf> {
    let relative_path = file_path.strip_prefix(content_root).unwrap_or(Path::new("."));
    let mut new_path = output_root.join(relative_path);
    new_path.set_extension("html");
    Ok(new_path)
}
