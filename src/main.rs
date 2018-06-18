pub mod css;
pub mod dom;
pub mod html;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn parse_css() {
    let mut source = File::open("test.css").unwrap();
    let mut css = String::new();
    source.read_to_string(&mut css).unwrap();
    let parsed = css::parse(css);
}

fn parse_html() {
    let mut html_file = File::open("test.html").unwrap();
    let mut contents = String::new();
    html_file.read_to_string(&mut contents).unwrap();
    let root = html::parse(contents);
}

fn main() {
    // TODO: implement flags to allow user to pass in file names
    // let args: Vec<String> = env::args().collect();

    parse_html();
    parse_css();
}
