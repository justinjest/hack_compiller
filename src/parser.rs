use std::fs;

pub fn open_line_breaks(filename: &str) -> String {
    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");
    return contents;
}


pub fn clean_whitespace(line: &str) -> &str {
    // We can split the line at // and select the first section to remove comments
    let vals = line.split("//").collect::<Vec<&str>>();
    vals[0].trim()
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

}
