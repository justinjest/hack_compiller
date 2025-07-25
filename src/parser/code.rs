use std::ops::Shl;

use std::collections::HashMap;

pub fn a_command (input: &str) -> u16 {
    // TODO: If this is includes a symbol we need to run it through the
    // command table to replace the value we need
    let val = &input[1..];
    let res:u16 = val.parse().unwrap();
    if res > 32767 {
        panic!("Unaddrasable area of memory accessed, crashing");
    }
    return res
}


pub fn c_command (val: &str) -> u16 {
    let (comp, dest, jmp) = split_c_command(val);
    let output = c_command_output(comp, dest, jmp);
    return output
}

fn split_c_command(val: &str) -> (&str, &str, &str) {
    // Split val by = to get section 1, and by ; to get section two and three
    // if 3 is "" return ""
    let comp: &str;
    let dest: &str;
    let jmp: &str;
    // This can be in the form of any of the following
    // a=b
    // a=b;c
    // a;c
    let first:Vec<_> = val.split(";").collect();
    if first.len() == 2 {
        jmp = first[1];
    } else {jmp = ""}
    let second:Vec<_> = first[0].split("=").collect();
    if second.len() == 2 {
        comp = second[1];
    } else {comp = ""}
    dest = second[0];

    return (comp, dest, jmp);
}

fn c_command_output(comp: &str, dest: &str, jmp: &str) -> u16 {
    let comp_section = create_comp(comp);
    let dest_section = create_dest(dest);
    let jmp_section = create_jmp(jmp);
    let res = combine_c(comp_section, dest_section, jmp_section);
    return res
}

fn combine_c(comp: u16, dest: u16, jmp: u16) -> u16 {
    // This uses bitwise ops to ensure that we are translating the
    // correct vals from the comp, dest, jmp tables
    // For C instructions the values will be
    // 111a_cccc_ccdd_djjj
    // For our implementation we are treating a_cccc_cc as one value
    let mut byte: u16 = 0b0000_0000_0000_0000;
    byte |= 0b1110_0000_0000_0000; // Header for c commands
    byte |= comp.shl(6);
    byte |= dest.shl(3);
    byte |= jmp;
    return byte;
}

// The following functions lookup what the int for the relevent symbol is
fn create_comp(comp: &str) -> u16 {
    let comp_table = HashMap::from([
        ("0",   0b0101010),
        ("1",   0b0111111),
        ("-1",  0b0111010),
        ("D",   0b0001100),
        ("A",   0b0110000),
        ("M",   0b1110000),
        ("!D",  0b0001101),
        ("!A",  0b0110001),
        ("!M",  0b1110001),
        ("-D",  0b0001111),
        ("-A",  0b0110011),
        ("-M",  0b1110011),
        ("D+1", 0b0011111),
        ("A+1", 0b0110111),
        ("M+1", 0b1110111),
        ("D-1", 0b0001110),
        ("A-1", 0b0110010),
        ("M-1", 0b1110010),
        ("D+A", 0b0000010),
        ("D+M", 0b1000010),
        ("D-A", 0b0010011),
        ("D-M", 0b1010011),
        ("A-D", 0b0000111),
        ("M-D", 0b1000111),
        ("D&A", 0b0000000),
        ("D&M", 0b1000000),
        ("D|A", 0b0010101),
        ("D|M", 0b1010101)
    ]);
    match comp_table.get(comp) {
        Some(val) => return *val,
        None => panic!("Invalid comp recieved")
    }
}

fn create_dest(dest: &str) -> u16 {
    let dest_table = HashMap::from([
        ("", 0),
        ("M", 1),
        ("D", 2),
        ("MD", 3),
        ("A", 4),
        ("AM", 5),
        ("AD", 6),
        ("AMD", 7),
    ]);
    match dest_table.get(dest) {
        Some(val) => return *val,
        // None could be replaced with an error to strengthen our program, however for this assighnment it is not needed
        None => return 0
    };
}

fn create_jmp(jmp: &str) -> u16 {
    let jmp_table = HashMap::from([
        ("", 0),
        ("JGT", 1),
        ("JEQ", 2),
        ("JGE", 3),
        ("JLT", 4),
        ("JNE", 5),
        ("JLE", 6),
        ("JMP", 7),
    ]);
    match jmp_table.get(jmp) {
        Some(val) => return *val,
        None => return 0
    }
}


pub fn l_command (_val: &str) -> u16 {
    return 0b0000_0000_0000_0010
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_command(){
        let res = a_command("@1");
        assert_eq!(res, 1)
    }

    #[test]
    fn test_a_command_max(){
        let res = a_command("@32767");
        assert_eq!(format!("{res:b}"), format!("{:b}", 32767))
    }

    #[test]
    #[should_panic]
    fn test_a_command_overflow(){
        let res = a_command("@32768");
        assert_eq!(format!("{res:b}"), format!("{:b}", 32767))
    }

    #[test]
    fn test_combine_c() {
        let res = combine_c(0, 0, 0);
        assert_eq!(res, 0b1110_0000_0000_0000);
    }

    #[test]
    fn test_combine_c_max() {
        let res = combine_c(85, 7, 7);
        assert_eq!(res, 0b1111_0101_0111_1111)
    }

    #[test]
    fn test_create_dest_null() {
        let res = create_dest(&"");
        assert_eq!(res, 0);
    }

    #[test]
    fn test_create_dest_m() {
        let res = create_dest(&"M");
        assert_eq!(res, 1);
    }

    #[test]
    fn test_create_dest_d() {
        let res = create_dest(&"D");
        assert_eq!(res, 2);
    }

    #[test]
    fn test_create_dest_md() {
        let res = create_dest(&"MD");
        assert_eq!(res, 3);
    }

    #[test]
    fn test_create_dest_a() {
        let res = create_dest(&"A");
        assert_eq!(res, 4);
    }

    #[test]
    fn test_create_dest_am() {
        let res = create_dest(&"AM");
        assert_eq!(res, 5);
    }

    #[test]
    fn test_create_dest_ad() {
        let res = create_dest(&"AD");
        assert_eq!(res, 6);
    }

    #[test]
    fn test_create_dest_amd() {
        let res = create_dest(&"AMD");
        assert_eq!(res, 7);
    }

    #[test]
    fn test_create_dest_error() {
        let res = create_dest(&"error");
        assert_eq!(res, 0);
    }

    #[test]
    fn test_create_jmp_null() {
        let res = create_jmp(&"");
        assert_eq!(res, 0);
    }

    #[test]
    fn test_create_jmp_jgt() {
        let res = create_jmp(&"JGT");
        assert_eq!(res, 1);
    }

    #[test]
    fn test_create_jmp_jeq() {
        let res = create_jmp(&"JEQ");
        assert_eq!(res, 2);
    }

    #[test]
    fn test_create_jmp_jge() {
        let res = create_jmp(&"JGE");
        assert_eq!(res, 3);
    }

    #[test]
    fn test_create_jmp_jlt() {
        let res = create_jmp(&"JLT");
        assert_eq!(res, 4);
    }

    #[test]
    fn test_create_jmp_jne() {
        let res = create_jmp(&"JNE");
        assert_eq!(res, 5);
    }

    #[test]
    fn test_create_jmp_jle() {
        let res = create_jmp(&"JLE");
        assert_eq!(res, 6);
    }

    #[test]
    fn test_create_jmp_jmp() {
        let res = create_jmp(&"JMP");
        assert_eq!(res, 0b111);
    }

    #[test]
    fn test_split_c_code_a() {
        let res = split_c_command("a=b;c");
        assert_eq!(res, ("b", "a", "c"));
    }

    #[test]
    fn test_split_c_code_b() {
        let res = split_c_command("a=b");
        assert_eq!(res, ("b", "a", ""));
    }

    #[test]
    fn test_split_c_code_c() {
        let res = split_c_command("a;c");
        assert_eq!(res, ("", "a", "c"));
    }

}
