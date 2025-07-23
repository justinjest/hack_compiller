mod parser;
mod code;


fn main() {
    println!("Hello, world!");
    let _ = parser::process_file("./resources/test.asm");
    let t = code::a_command("");
    print!("{t}");
}
