pub mod dom;
pub mod html;

use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut f = File::open(&args[1]).unwrap(); // ? replaces .expect; on panic!, ? returns error value from current function

    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    
    println!("{}", contents);

    let root = html::parse(contents);

    println!("{:?}", root.node_type);
}
