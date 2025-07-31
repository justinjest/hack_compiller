use std::fs;
pub mod parser;

fn main() {
   parser::process_file("./resources/pong.asm");
    let c = fs::read_to_string("./resources/pong.hack")
        .expect("Unable to parse created");
    let e = fs::read_to_string("./resources/pong_correct.hack")
        .expect("Unable to parse written");
    assert_eq!(c, e);
}
