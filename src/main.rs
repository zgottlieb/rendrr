pub mod css;
pub mod dom;
pub mod html;
pub mod style;

// use std::env;
use std::fs::File;
use std::io::prelude::*;

fn parse_css() -> css::Stylesheet {
    let mut source = File::open("test.css").unwrap();
    let mut css = String::new();
    source.read_to_string(&mut css).unwrap();
    css::parse(css)
}

fn parse_html() -> dom::Node {
    let mut html_file = File::open("test.html").unwrap();
    let mut contents = String::new();
    html_file.read_to_string(&mut contents).unwrap();
    html::parse(contents)
}

fn main() {
    // TODO: implement flags to allow user to pass in file names
    // let args: Vec<String> = env::args().collect();

    let html = parse_html();
    let stylesheet = parse_css();

    let style_tree = style::build_style_tree(&html, &stylesheet);

    println!("{:#?}", html);
    println!("{:#?}", stylesheet);
    println!("{:#?}", style_tree);
}
