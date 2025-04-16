use std::{collections::HashMap, fs, io};

/// Represents a registry of HTML templates
pub struct TemplateRegistry {
    templates: HashMap<String, String>,
}

impl TemplateRegistry {
    pub fn new() -> Self {
        Self { templates: HashMap::new() }
    }

    /// Load the HTML files in the specified directory into the template registry
    pub fn load_from_dir(&mut self, path: &str) -> io::Result<()> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("html") {
                let name = match path.file_stem() {
                    Some(name) => name.to_string_lossy().to_string(),
                    None => continue,
                };
                let contents = fs::read_to_string(&path)?;
                self.templates.insert(name, contents);
            }
        }

        Ok(())
    }
}

impl Default for TemplateRegistry {
    fn default() -> Self {
        TemplateRegistry::new()
    }
}
