use crate::file;

const BASE_HTML: &str = include_str!("template_files/base.html");
const README: &str = include_str!("template_files/README.md");
const RUSTYPAGES_TOML: &str = include_str!("template_files/rustypages.toml");

const FILES: &[(&str, &str)] = &[
    ("templates/base.html", BASE_HTML),
    ("README.md", README),
    ("rustypages.toml", RUSTYPAGES_TOML),
];

pub fn init() {
    println!("Initializing new RustyPages project...");

    for (path, content) in FILES.iter() {
        if let Err(error) = file::write_file(path, content) {
            println!("Error creating file {}: {}", path, error);
            return;
        }

        println!("Created file {}", path);
    }
}

pub fn build() {
    println!("Building the site...");
}

pub fn clean() {
    println!("Cleaning output directory...");
}
