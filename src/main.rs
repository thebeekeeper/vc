#[macro_use]
extern crate prettytable;
extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;

use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::{PrecClimber, Assoc, Operator};

use std::env;
use prettytable::Table;

#[derive(Parser)]
#[grammar = "calc.pest"]
struct CalcParser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Not enough args");
        println!("TODO: write an error message");
        return;
    }
    
    if args[1] == "-r" {
        convert_radix(args[2].clone());
    }
    else {
        let stmt = args[1..].join(" ");
        let pairs = CalcParser::parse(Rule::calculation, &stmt).unwrap();
        let r = eval(pairs);
        println!("{} = {}", stmt, r);
    }
}

fn convert_radix(val: String) {
    let mut radix = 10;
    let mut v = val;
    if v.contains("0x") {
        radix = 16;
        v = v.trim_left_matches("0x").to_string();
    }
    let uval = i32::from_str_radix(&v, radix).expect(&format!("Failed to convert {} to an integer", v));

    let mut table = Table::new();
    table.add_row(row!["Decimal", "Hex", "Octal", "ASCII", "Binary"]);
    table.add_row(row![uval, format!("{:X}", uval), format!("{:o}", uval), ((uval as u8) as char), format!("{:b}", uval)]);
    table.printstd();
}

lazy_static! {
   static ref PREC_CLIMBER: PrecClimber<Rule> = {
      use Rule::*;
      use Assoc::*;

      PrecClimber::new(vec![
         Operator::new(add, Left) | Operator::new(subtract, Left),
         Operator::new(multiply, Left) | Operator::new(divide, Left),
         Operator::new(power, Right)
      ])
   };
}

fn eval(expression: Pairs<Rule>) -> f64 {
   PREC_CLIMBER.climb(
     expression,
     |pair: Pair<Rule>| match pair.as_rule() {
         Rule::num => pair.as_str().parse::<f64>().unwrap(),
         Rule::expr => eval(pair.into_inner()),
         _ => unreachable!(),
      },
      |lhs: f64, op: Pair<Rule>, rhs: f64| match op.as_rule() {
         Rule::add      => lhs + rhs,
         Rule::subtract => lhs - rhs,
         Rule::multiply => lhs * rhs,
         Rule::divide   => lhs / rhs,
         Rule::power    => lhs.powf(rhs),
         _ => unreachable!(),
      },
   )
}

