use std::collections::HashMap;

use clap::Parser;
use itertools::Itertools;
use lazy_static::lazy_static;

// There's probably a nicer way to do this while maintaining my existing directory structure,
// but I'm doing this hackily for now to avoid spending a lot of time on this.

#[path = "./Day 01/Rust/sol.rs"] mod one;
#[path = "./Day 02/Rust/sol.rs"] mod two;
#[path = "./Day 03/Rust/sol.rs"] mod three;
#[path = "./Day 04/Rust/sol.rs"] mod four;
#[path = "./Day 05/Rust/sol.rs"] mod five;
#[path = "./Day 06/Rust/sol.rs"] mod six;
#[path = "./Day 07/Rust/sol.rs"] mod seven;
#[path = "./Day 08/Rust/sol.rs"] mod eight;
#[path = "./Day 09/Rust/sol.rs"] mod nine;
#[path = "./Day 10/Rust/sol.rs"] mod ten;
#[path = "./Day 11/Rust/sol.rs"] mod eleven;
#[path = "./Day 12/Rust/sol.rs"] mod twelve;
#[path = "./Day 13/Rust/sol.rs"] mod thirteen;
#[path = "./Day 14/Rust/sol.rs"] mod fourteen;
#[path = "./Day 15/Rust/sol.rs"] mod fifteen;
#[path = "./Day 16/Rust/sol.rs"] mod sixteen;
#[path = "./Day 17/Rust/sol.rs"] mod seventeen;
//#[path = "./Day 18/Rust/sol.rs"] mod eighteen;
//#[path = "./Day 19/Rust/sol.rs"] mod nineteen;
//#[path = "./Day 20/Rust/sol.rs"] mod twenty;
//#[path = "./Day 21/Rust/sol.rs"] mod twentyOne;
//#[path = "./Day 22/Rust/sol.rs"] mod twentyTwo;
//#[path = "./Day 23/Rust/sol.rs"] mod twentyThree;
//#[path = "./Day 24/Rust/sol.rs"] mod twentyFour;
//#[path = "./Day 25/Rust/sol.rs"] mod twentyFive;

type Func = fn() -> ();
lazy_static! {
    static ref sols: HashMap<u8, Func> =
    #[allow(clippy::zero_prefixed_literal)] {
        let mut m = HashMap::new();
        m.insert(01u8, one::solve as Func);
        m.insert(02u8, two::solve as Func);
        m.insert(03u8, three::solve as Func);
        m.insert(04u8, four::solve as Func);
        m.insert(05u8, five::solve as Func);
        m.insert(06u8, six::solve as Func);
        m.insert(07u8, seven::solve as Func);
        m.insert(08u8, eight::solve as Func);
        m.insert(09u8, nine::solve as Func);
        m.insert(10u8, ten::solve as Func);
        m.insert(11u8, eleven::solve as Func);
        m.insert(12u8, twelve::solve as Func);
        m.insert(13u8, thirteen::solve as Func);
        m.insert(14u8, fourteen::solve as Func);
        m.insert(15u8, fifteen::solve as Func);
        m.insert(16u8, sixteen::solve as Func);
        m.insert(17u8, seventeen::solve as Func);
//        m.insert(18u8, eighteen::solve as Func);
//        m.insert(19u8, nineteen::solve as Func);
//        m.insert(20u8, twenty::solve as Func);
//        m.insert(21u8, twentyOne::solve as Func);
//        m.insert(22u8, twentyTwo::solve as Func);
//        m.insert(23u8, twentyThree::solve as Func);
//        m.insert(24u8, twentyFour::solve as Func);
//        m.insert(25u8, twentyFive::solve as Func);
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
            sols[day]();
        }
    }
}
