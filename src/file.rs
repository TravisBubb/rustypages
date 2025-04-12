use std::io::{self, Write};

/// Creates a new directory with the given path if it does not already exist
pub fn create_dir_if_not_exists(path: &str) -> io::Result<()> {
    let dir_path = std::path::Path::new(path);

    if !dir_path.try_exists()? {
        std::fs::create_dir_all(dir_path)?;
    }

    Ok(())
}

/// Creates a file if it does not exist and writes the given contents to it
pub fn write_file(path: &str, contents: &str) -> io::Result<()> {
    let file_path = std::path::Path::new(path);

    if !file_path.try_exists()? {
        if let Some(dir) = file_path.parent() {
            create_dir_if_not_exists(dir.to_str().unwrap_or_default())?;
        }
        let mut file = std::fs::File::create(file_path)?;
        file.write_all(contents.as_bytes())?;
    }

    Ok(())
}
