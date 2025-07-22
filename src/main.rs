mod utils;

use crate::utils::io;

fn main() {
    println!("Hello, world!");
    io::open_line_breaks("./resources/test.asm");
}
