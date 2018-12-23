#[macro_use]
extern crate prettytable;

use std::env;
use prettytable::Table;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("ERROR!  Input value required.");
        println!("\tValue with no prefix will be treated as a deicimal");
        println!("\tValue with a 0x prefix will be treated as hex");
        return;
    }
    let mut val = args[1].clone();
    let mut radix = 10;
    if val.contains("0x") {
        radix = 16;
        val = val.trim_left_matches("0x").to_string();
    }
    let uval = i32::from_str_radix(&val, radix).expect(&format!("Failed to convert {} to an integer", val));

    let mut table = Table::new();
    table.add_row(row!["Decimal", "Hex", "Octal", "ASCII", "Binary"]);
    table.add_row(row![uval, format!("{:X}", uval), format!("{:o}", uval), ((uval as u8) as char), format!("{:b}", uval)]);
    table.printstd();
}
