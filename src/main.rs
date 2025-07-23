mod utils;
mod parser;
mod code;

use crate::utils::io;

fn main() {
    println!("Hello, world!");
    let _ = parser::process_file("./resources/test.asm");
    let t = code::a_command("");
    print!("{t}");
}
