use pulldown_cmark::{html, Options, Parser};

pub fn get_default_options() -> Options {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_MATH);
    options.insert(Options::ENABLE_GFM);
    options
}

pub fn convert_to_html(content: &str, options: Option<Options>) -> String {
    let opt = options.unwrap_or(get_default_options());
    let parser = Parser::new_ext(content, opt);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
