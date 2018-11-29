use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut val = args[1].clone();
    let mut radix = 10;
    if val.contains("0x") {
        radix = 16;
        val = val.trim_left_matches("0x").to_string();
    }
    let uval = i32::from_str_radix(&val, radix).expect(&format!("Failed to convert {} to an integer", val));
    println!("Decimal     Hex          Octal        ASCII    Binary");
    println!("{n:<width$}{h:#010X}   {o:#010o}   {a}        {b:#034b}", n=uval, width=12, h=uval, o=uval, a=((uval as u8) as char), b=uval);
}
