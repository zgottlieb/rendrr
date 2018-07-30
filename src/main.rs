extern crate sdl2;
extern crate getopts;

use getopts::Options;
use std::env;

pub mod css;
pub mod dom;
pub mod html;
pub mod style;
pub mod layout;
pub mod painting;
pub mod sdlbackend;

use std::fs::File;
use std::io::prelude::*;

fn parse_css(file_name: &String) -> css::Stylesheet {
    let mut source = File::open(file_name).unwrap();
    let mut css = String::new();
    source.read_to_string(&mut css).unwrap();
    css::parse(css)
}

fn parse_html(file_name: &String) -> dom::Node {
    let mut html_file = File::open(file_name).unwrap();
    let mut contents = String::new();
    html_file.read_to_string(&mut contents).unwrap();
    html::parse(contents)
}

fn usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} -h FILE -c FILE", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().map(|x| x.to_string()).collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("H", "html", "Input HTML file name", "HTML");
    opts.optopt("c", "css", "Input CSS file name", "CSS");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    
    if matches.opt_present("h") {
        usage(&program, opts);
        return;
    }

    let html_file = match matches.opt_str("H") {
        Some(x) => { x }
        None => { panic!("No HTML file selected") },
    };

    let css_file = match matches.opt_str("c") {
        Some(x) => { x }
        None => { panic!("No CSS file selected") }
    };

    let context = sdlbackend::init();
    let window = sdlbackend::window(&context);
    let mut viewport: layout::Dimensions = Default::default();
    let (width, height) = window.position();
    viewport.content.width = width as f32;
    viewport.content.height = height as f32;

    let html = parse_html(&html_file);
    let stylesheet = parse_css(&css_file);
    let style_tree = style::build_style_tree(&html, &stylesheet);
    let layout_root = layout::layout_tree(&style_tree, viewport);
    let display_list = painting::build_display_list(&layout_root);
    sdlbackend::render(&context, window);

    // println!("{:#?}", html);
    // println!("{:#?}", stylesheet);
    // println!("{:#?}", style_tree);
    // println!("{:#?}", layout_root);
    println!("{:#?}", display_list);
}
