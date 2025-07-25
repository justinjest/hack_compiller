use crate::parser::code;

pub mod parser;

fn main() {
    println!("Hello, world!");
    let _ = parser::process_file("./resources/test.asm");
    let t = code::a_command("@32767");
    print!("{}", format!("{:016b}", t));
}
