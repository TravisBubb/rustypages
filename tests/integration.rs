use rustypages::builder;
use std::fs;

#[test]
fn simple_template_is_applied_to_markdown() {
    let temp_dir = tempfile::tempdir().unwrap();
    let site_dir = temp_dir.path();

    fs::create_dir(site_dir.join("content")).unwrap();
    fs::create_dir(site_dir.join("templates")).unwrap();

    fs::write(site_dir.join("content").join("index.md"), "# Hello\nThis is a test.").unwrap();

    fs::write(
        site_dir.join("templates").join("layout.html"),
        "<html><body>{{ content }}</body></html>",
    )
    .unwrap();

    builder::build(site_dir).unwrap();

    let output_path = site_dir.join("dist").join("index.html");
    assert!(output_path.exists());

    let html = fs::read_to_string(output_path).unwrap();
    assert!(html.contains("<html>"));
    assert!(html.contains("<h1>Hello</h1>"));
    assert!(html.contains("This is a test."));
}

#[test]
fn multiple_pages_are_rendered_correctly() {
    let temp_dir = tempfile::tempdir().unwrap();
    let site_dir = temp_dir.path();

    fs::create_dir_all(site_dir.join("content/blog")).unwrap();
    fs::create_dir(site_dir.join("templates")).unwrap();

    fs::write(site_dir.join("content").join("index.md"), "# Home").unwrap();
    fs::write(site_dir.join("content").join("about.md"), "# About Me").unwrap();
    fs::write(site_dir.join("content/blog").join("post1.md"), "# First Post").unwrap();
    fs::write(site_dir.join("content/blog").join("post2.md"), "# Second Post").unwrap();

    fs::write(
        site_dir.join("templates").join("layout.html"),
        "<html><body>{{ content }}</body></html>",
    )
    .unwrap();

    builder::build(site_dir).unwrap();

    let files =
        ["dist/index.html", "dist/about.html", "dist/blog/post1.html", "dist/blog/post2.html"];

    for file in files.iter() {
        let path = site_dir.join(file);
        assert!(path.exists(), "Expected {} to exist", file);
        let contents = fs::read_to_string(&path).unwrap();
        assert!(contents.contains("<html>"), "Missing html tag in {}", file);
    }
}
