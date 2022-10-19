use std::{
    fs::File,
    io::{Cursor, Read},
};

use rustache::{HashBuilder, Render};

fn main() {
    let data = HashBuilder::new().insert("name", "rust111");
    let mut out = Cursor::new(Vec::new());
    let mut file = File::open("src/templates/data.mustache").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    data.render(&contents, &mut out).unwrap();
    println!("{}", String::from_utf8(out.into_inner()).unwrap());
}
