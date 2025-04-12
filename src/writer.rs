use pulldown_cmark::{Options, Parser, html};

pub fn write_html_from_markdown(content: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS | Options::ENABLE_TABLES);

    let parser = Parser::new_ext(content, options);

    let mut html = String::new();
    html::push_html(&mut html, parser);
    html
}
