mod utils;

use crate::utils::io;

fn main() {
    println!("Hello, world!");
    io::open_line_breaks("./resources/test.asm");
    let _ = io::write_file("./resources/test_output.hack", "hello, rust");
}
