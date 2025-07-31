use std::ops::Shl;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    symbol_table:HashMap<String, u16>,
    next_rom: u16,
}

impl Table {
    pub fn new() -> Self {
        let initial = HashMap::from([
            ("SP".into(), 0),
            ("LCL".into(), 1),
            ("ARG".into(), 2),
            ("THIS".into(), 3),
            ("THAT".into(), 4),
            ("R0".into(), 0),
            ("R1".into(), 1),
            ("R2".into(), 2),
            ("R3".into(), 3),
            ("R4".into(), 4),
            ("R5".into(), 5),
            ("R6".into(), 6),
            ("R7".into(), 7),
            ("R8".into(), 8),
            ("R9".into(), 9),
            ("R10".into(), 10),
            ("R11".into(), 11),
            ("R12".into(), 12),
            ("R13".into(), 13),
            ("R14".into(), 14),
            ("R15".into(), 15),
            ("SCREEN".into(), 16384),
            ("KBD".into(), 24576),
        ]);
            Table { symbol_table: initial, next_rom: 16 }
    }

    fn insert(&mut self, name: String, line: u16) {
        self.symbol_table.insert(name, line.into());
    }

    fn get_addr(&mut self, name: &str) -> u16 {
        if let Some(&addr) = self.symbol_table.get(name) {
            addr as u16
        } else {
            // TK This is horid and needs to be improved
            0
        }
    }

    fn exists(&mut self, name: &str) -> bool {
        self.symbol_table.contains_key(name)
    }

    fn get_next_rom(&mut self) -> u16 {
        let cur = self.next_rom;
        self.next_rom += 1;
        cur
    }
}

pub fn a_command (input: &str, table: &mut Table) -> u16 {
    // TODO: If this is includes a symbol we need to run it through the
    // command table to replace the value we need
    let s = &input[1..];
    match s.parse::<u16>() {
        Ok(n) => return n,
        Err(_) => if table.exists(&s) {
            let val = table.get_addr(&s);
            return val;
        } else {
            let rom = table.get_next_rom();
            table.insert(s.to_string(), rom);
            let val = rom;
            return val;
        }
    }
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
    let mut dest: &str = "";
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
        dest = second[0];
    } else {comp = second[0]}
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
        None => panic!("Invalid comp recieved {comp}")
    }
}

fn create_dest(dest: &str) -> u16 {
    let dest_table = HashMap::from([
        ("", 0),
        ("M", 1),
        ("D", 2),
        ("MD", 3),
        ("DM", 3),
        ("A", 4),
        ("AM", 5),
        ("AD", 6),
        ("AMD", 7),
        ("ADM", 7),
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

pub fn first_pass_l(line: &str, line_num: &mut u16, table: &mut Table) {
    let s = line.replace(&['(', ')'][..], "");
    table.insert(s, *line_num);
}

pub fn l_command (val: &str, table: &mut Table) -> u16 {
    table.get_addr(val)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_command(){
        let res = a_command("@1", &mut Table::new());
        assert_eq!(res, 1)
    }

    #[test]
    fn test_a_command_max(){
        let res = a_command("@32767", &mut Table::new());
        assert_eq!(format!("{res:b}"), format!("{:b}", 32767))
    }

    #[test]
    #[should_panic]
    fn test_a_command_overflow(){
        let res = a_command("@32768", &mut Table::new());
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
        assert_eq!(res, ("a", "", "c"));
    }

    #[test]
    fn test_default_map() {
        let m = Table::new();

        let v = HashMap::from([
            ("SP".into(), 0),
            ("LCL".into(), 1),
            ("ARG".into(), 2),
            ("THIS".into(), 3),
            ("THAT".into(), 4),
            ("R0".into(), 0),
            ("R1".into(), 1),
            ("R2".into(), 2),
            ("R3".into(), 3),
            ("R4".into(), 4),
            ("R5".into(), 5),
            ("R6".into(), 6),
            ("R7".into(), 7),
            ("R8".into(), 8),
            ("R9".into(), 9),
            ("R10".into(), 10),
            ("R11".into(), 11),
            ("R12".into(), 12),
            ("R13".into(), 13),
            ("R14".into(), 14),
            ("R15".into(), 15),
            ("SCREEN".into(), 16384),
            ("KBD".into(), 24576),
        ]);
        assert_eq!(m.symbol_table, v);
    }

    #[test]
    fn test_add_to_map() {
        let mut m = Table::new();
        m.insert("1".to_string(), 1);
        let v = HashMap::from([
            ("SP".into(), 0),
            ("LCL".into(), 1),
            ("ARG".into(), 2),
            ("THIS".into(), 3),
            ("THAT".into(), 4),
            ("R0".into(), 0),
            ("R1".into(), 1),
            ("R2".into(), 2),
            ("R3".into(), 3),
            ("R4".into(), 4),
            ("R5".into(), 5),
            ("R6".into(), 6),
            ("R7".into(), 7),
            ("R8".into(), 8),
            ("R9".into(), 9),
            ("R10".into(), 10),
            ("R11".into(), 11),
            ("R12".into(), 12),
            ("R13".into(), 13),
            ("R14".into(), 14),
            ("R15".into(), 15),
            ("SCREEN".into(), 16384),
            ("KBD".into(), 24576),
            ("1".into(), 1),
        ]);
        assert_eq!(m.symbol_table, v);

        let q = m.exists("1");
        let r = true;
        assert_eq!(q, r);

        let q = m.get_addr("1");
        let r = 1;
        assert_eq!(q,r);
    }

    #[test]
    fn test_first_pass_l() {
        let mut t = Table::new();
        let mut line_num = 1 as u16;
        let val = "(some)";
        first_pass_l(&val, &mut line_num, &mut t);
        let mut e = Table::new();
        e.insert("some".to_string(), 2);

        assert_eq!(t.symbol_table, e.symbol_table);
    }

    #[test]
    fn failing_comp() {
        let r = c_command("D;JGT");
        print!("{r}");
        let e = 0b1110001100000001 as u16;
        assert_eq!(r, e);

    }
}
