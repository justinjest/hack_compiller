use std::fs;
use std::io::{Result, Write};

pub mod code;

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
    let array2 = array.clone();
    let mut table = code::Table::new();
    let mut res = Vec::new();
    let mut line_num:u16 = 0;
    for i in array2 {
        let tmp = clean_whitespace(i);
        if tmp.is_some() {
            first_pass_line(tmp.unwrap(), &mut line_num, &mut table);
        }
    }
    for i in array {
        let tmp = clean_whitespace(i);
        if tmp.is_some() {
            let (_, output) = process_line(&mut line_num, tmp.unwrap(),
                                           &mut table);
            if output != u16::MAX {
                res.push(output);
            }
        }
    }
    let output = translate_bits_into_string(res);
    let _ = write_file(&isolate_filename(filename), &output);
}

fn first_pass_line<'a>(line:  &str,
                       line_num: &mut u16,
                       table: &mut code::Table) {
    if line.starts_with('@') {
        *line_num += 1;
    } else if line.starts_with('(') {
        code::first_pass_l(line, line_num, table);
    } else{
        *line_num += 1;
    }
}

fn translate_bits_into_string(vals: Vec<u16>) -> String {
    let mut tmp = Vec::new();
    for i in vals {
        tmp.push(format!("{:016b}", i));
    }
    let mut res = tmp.join("\n");
    res = res + "\n";
    res
}

pub fn process_line(line_num: &mut u16,
                        line: &str,
                        table: &mut code::Table) ->(u16, u16) {
    if line.starts_with('@') {
        *line_num += 1;
        return (*line_num, code::a_command(line, table));
    } else if line.starts_with('(') {
        return (*line_num, u16::MAX);
    } else {
        *line_num += 1;
        return (*line_num, code::c_command(line));
    }
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
        assert_eq!(result, Some("@1"));
    }

    #[test]
    fn test_clean_whitespace_whole() {
        let result = clean_whitespace("//Test");
        assert_eq!(result, None);
    }

    #[test]
    fn test_clean_whitespace_no_change() {
        let result = clean_whitespace("Test");
        assert_eq!(result, Some("Test"));
    }

    #[test]
    fn test_clean_whitespace_no_content() {
        let result = clean_whitespace("");
        assert_eq!(result, None);
    }
    #[test]
    fn test_isolate_filename(){
        let res = isolate_filename("./resources/test.asm");
        assert_eq!(res, "./resources/test.hack");
    }

    #[test]
    fn test_process_line_a(){
        let mut line_num = 1 as u16;
        let mut t = code::Table::new();
        let res = process_line(&mut line_num, &"@2", &mut t);
        assert_eq!(res, (2 as u16, 0b0000_0000_0000_0010 as u16));
    }

    #[test]
    fn test_process_line_c1(){
        let mut line_num = 1 as u16;
        let mut t = code::Table::new();
        let res = process_line(&mut line_num, &"D=A", &mut t);
        assert_eq!(res, (2 as u16, 0b1110_1100_0001_0000 as u16));
    }

}
