mod utils;
mod parser;

use crate::utils::io;

fn main() {
    println!("Hello, world!");
    let _ = parser::process_file("./resources/test.asm");
}
