use std::fs;
use std::io::{Result, Write};
use std::option;

pub fn open_line_breaks(filename: &str) -> String {
    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");
    return contents;
}

pub fn write_file (filename: &str, contents: &str) -> Result<()> {
    let mut file = fs::File::create(format!("{}", filename.to_string()))?;
    let _ = file.write_all(contents.as_bytes());
    Ok(())
}

pub fn clean_whitespace(line: &str) -> Option<&str> {
    // We can split the line at // and select the first section to remove comments
    let vals = line.split("//").collect::<Vec<&str>>();
    let tmp = vals[0].trim();
    if tmp != "" {
        return Some(tmp)
    }
    return None
}

pub fn process_file(filename: &str) {
    let contents = open_line_breaks(filename);
    let array = contents.split("\n");
    let mut res = Vec::new();
    for i in array {
        let tmp = clean_whitespace(i);
        if tmp.is_some() {
            res.push(tmp.unwrap());
        }
    }
    let output = res.join("\n");
    let _ = write_file(&isolate_filename(filename), &output);
}

fn isolate_filename(filepath: &str) -> String {
    let mut array = filepath.split("/").peekable();
    let mut filepath = Vec::new();
    while array.peek() != None {
        let tmp = array.next().unwrap();
        if array.peek() == None {
            let mut filename = tmp.splitn(2, ".");
            filepath.push(format!("{}{}", filename.next().unwrap(), ".hack"));
        } else {
            filepath.push(tmp.to_string());
        }
    }
    let res = filepath.join("/");
    return res.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_whitespace_inline() {
        let result = clean_whitespace("@1 //Test");
        assert_eq!(result, "@1");
    }

    #[test]
    fn test_clean_whitespace_whole() {
        let result = clean_whitespace("//Test");
        assert_eq!(result, "");
    }

    #[test]
    fn test_clean_whitespace_no_change() {
        let result = clean_whitespace("Test");
        assert_eq!(result, "Test");
    }

    #[test]
    fn test_clean_whitespace_no_content() {
        let result = clean_whitespace("");
        assert_eq!(result, "");
    }
    #[test]
    fn test_isolate_filename(){
        let res = isolate_filename("./resources/test.asm");
        assert_eq!(res, "./resources/test.hack");
    }
}
