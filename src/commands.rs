use crate::{builder, file};

const BASE_HTML: &str = include_str!("template_files/base.html");
const README: &str = include_str!("template_files/README.md");
const RUSTYPAGES_TOML: &str = include_str!("template_files/rustypages.toml");

const PROJECT_FILES: &[(&str, &str)] = &[
    ("templates/base.html", BASE_HTML),
    ("README.md", README),
    ("rustypages.toml", RUSTYPAGES_TOML),
];

pub fn init() {
    println!("Initializing new RustyPages project...");

    for (path, content) in PROJECT_FILES.iter() {
        if let Err(error) = file::write_file(path, content) {
            println!("Error creating file {}: {}", path, error);
            return;
        }

        println!("Created file {}", path);
    }
}

pub fn build() {
    // TODO: Validate that the current directory is a rustypages project

    println!("Building the site...");

    if let Err(error) = builder::build() {
        println!("Failed to build site: {}", error);
    }

}

pub fn clean() {
    println!("Cleaning output directory...");
}
