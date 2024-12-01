use std::collections::HashMap;

use clap::{Parser};
use itertools::Itertools;
use lazy_static::lazy_static;

// There's probably a nicer way to do this while maintaining my existing directory structure,
// but I'm doing this hackily for now to avoid spending a lot of time on this.

#[path = "./Day 01/Rust/sol.rs"] mod one;
//#[path = "./Day 02/Rust/sol.rs"] mod two;
//#[path = "./Day 03/Rust/sol.rs"] mod Three;
//#[path = "./Day 04/Rust/sol.rs"] mod Four;
//#[path = "./Day 05/Rust/sol.rs"] mod Five;
//#[path = "./Day 06/Rust/sol.rs"] mod Six;
//#[path = "./Day 07/Rust/sol.rs"] mod Seven;
//#[path = "./Day 08/Rust/sol.rs"] mod Eight;
//#[path = "./Day 09/Rust/sol.rs"] mod Nine;
//#[path = "./Day 10/Rust/sol.rs"] mod Ten;
//#[path = "./Day 11/Rust/sol.rs"] mod Eleven;
//#[path = "./Day 12/Rust/sol.rs"] mod Twelve;
//#[path = "./Day 13/Rust/sol.rs"] mod Thirteen;
//#[path = "./Day 14/Rust/sol.rs"] mod Fourteen;
//#[path = "./Day 15/Rust/sol.rs"] mod Fifteen;
//#[path = "./Day 16/Rust/sol.rs"] mod Sixteen;
//#[path = "./Day 17/Rust/sol.rs"] mod Seventeen;
//#[path = "./Day 18/Rust/sol.rs"] mod Eighteen;
//#[path = "./Day 19/Rust/sol.rs"] mod Nineteen;
//#[path = "./Day 20/Rust/sol.rs"] mod Twenty;
//#[path = "./Day 21/Rust/sol.rs"] mod TwentyOne;
//#[path = "./Day 22/Rust/sol.rs"] mod TwentyTwo;
//#[path = "./Day 23/Rust/sol.rs"] mod TwentyThree;
//#[path = "./Day 24/Rust/sol.rs"] mod TwentyFour;
//#[path = "./Day 25/Rust/sol.rs"] mod TwentyFive;

type Func = fn() -> ();
lazy_static! {
    static ref sols: HashMap<u8, Func> = {
        let mut m = HashMap::new();
        m.insert(01u8, one::solve as Func);
//        m.insert(02u8, two::solve as Func);
//        m.insert(03u8, Three::solve as Func)
//        m.insert(04u8, Four::solve as Func)
//        m.insert(05u8, Five::solve as Func)
//        m.insert(06u8, Six::solve as Func)
//        m.insert(07u8, Seven::solve as Func)
//        m.insert(08u8, Eight::solve as Func)
//        m.insert(09u8, Nine::solve as Func)
//        m.insert(10u8, Ten::solve as Func)
//        m.insert(11u8, Eleven::solve as Func)
//        m.insert(12u8, Twelve::solve as Func)
//        m.insert(13u8, Thirteen::solve as Func)
//        m.insert(14u8, Fourteen::solve as Func)
//        m.insert(15u8, Fifteen::solve as Func)
//        m.insert(16u8, Sixteen::solve as Func)
//        m.insert(17u8, Seventeen::solve as Func)
//        m.insert(18u8, Eighteen::solve as Func)
//        m.insert(19u8, Nineteen::solve as Func)
//        m.insert(20u8, Twenty::solve as Func)
//        m.insert(21u8, TwentyOne::solve as Func)
//        m.insert(22u8, TwentyTwo::solve as Func)
//        m.insert(23u8, TwentyThree::solve as Func)
//        m.insert(24u8, TwentyFour::solve as Func)
//        m.insert(25u8, TwentyFive::solve as Func)
        m
    };
}

#[derive(Parser)]
struct Opts {
    #[arg(short, long, value_name = "DATE")]
    day: Option<u8>,
}

fn main() {
    let opts = Opts::parse();
    
    if opts.day.is_some() {
        let day = opts.day.unwrap();
        if sols.contains_key(&day) {
            sols[&day]();
        } else {
            println!("Invalid day input - no solution exists for day {}", day);
        }
    } else {
        for day in sols.keys().sorted() {
            sols[&day]();
        }
    }
}