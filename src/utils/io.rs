use std::fs;

pub fn open_line_breaks(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");
    let mut res = Vec::new();
    for i in contents.lines() {
        print!("{i}\n");
        res.push(i.to_string());
    }
    return res;
}
