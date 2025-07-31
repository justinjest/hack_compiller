use crate::parser::code;

pub mod parser;

fn main() {
    println!("Hello, world!");
    let _ = parser::process_file("./resources/test.asm");
}
