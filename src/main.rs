mod aoc;
mod day_1;
mod day_2;
mod day_3;

pub use crate::aoc::*;
pub use crate::day_1::*;
pub use crate::day_2::*;
pub use crate::day_3::*;

fn main() {
    let d1 = AoC2023::<1, &str>::default();
    let r1 = d1.eval();
    println!("{:?}", r1);
    let d2 = AoC2023::<2, &str>::default();
    let r2 = d2.eval();
    println!("{:?}", r2);
    let d3 = AoC2023::<3, &str>::default();
    let r3 = d3.eval();
    println!("{:?}", r3);
}
