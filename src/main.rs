mod aoc;
mod day_1;

pub use crate::aoc::*;
pub use crate::day_1::*;

fn main() {
    let d1 = AoC2023::<1, &str>::default();
    let r1 = d1.eval();
    println!("{:?}", r1);
}
