use regex::Regex;
use statrs::statistics::Statistics;
use std::time::Instant;

#[derive(Debug)]
pub struct AoC2023<const DAY: u32, T: std::fmt::Debug> {
    pub input: T,
}

#[derive(Debug)]
pub struct DayEval {
    pub mean: f64,
    pub std: f64,
    pub answer: Option<String>,
}

pub trait Day {
    fn default() -> Self;

    fn part_one(&self) -> Option<String> {
        None
    }

    fn part_two(&self) -> Option<String> {
        None
    }

    fn eval_part(&self, uses_part_one: bool) -> DayEval {
        let mut times = Vec::<f64>::new();
        let mut answers = Vec::<String>::new();

        for _ in 0..30 {
            let t0 = Instant::now();
            let result: Option<String>;
            if uses_part_one {
                result = self.part_one();
            } else {
                result = self.part_two();
            }
            times.push((Instant::now() - t0).as_nanos() as f64 / 1e3);
            match result {
                Some(string) => answers.push(string),
                None => (),
            };
        }

        let ans = {
            if let Some(first_element) = answers.first() {
                match answers.iter().all(|element| element == first_element) {
                    true => Some(first_element.clone()),
                    false => None,
                }
            } else {
                None
            }
        };

        DayEval {
            mean: times.clone().mean(),
            std: times.clone().std_dev(),
            answer: ans,
        }
    }

    fn eval(&self) -> (DayEval, DayEval) {
        (self.eval_part(true), self.eval_part(false))
    }
}
fn parse_str(text: &str) -> u32 {
    let mut sum = 0u32;
    for line in text.lines() {
        let mut c0 = String::new();
        for c in line.chars() {
            if c >= '0' && c <= '9' {
                c0.push(c);
                break;
            }
        }
        for c in line.chars().rev() {
            if c >= '0' && c <= '9' {
                c0.push(c);
                break;
            }
        }
        sum += c0.parse::<u32>().unwrap();
    }
    return sum;
}

impl Day for AoC2023<1, &str> {
    fn default() -> Self {
        Self {
            input: include_str!("../data/input1.txt"),
        }
    }

    fn part_one(&self) -> Option<String> {
        Some(parse_str(&self.input).to_string())
    }

    fn part_two(&self) -> Option<String> {
        let mut converted = self.input.to_string();
        let result = converted
            .lines()
            .map(|line| {
                let mut digit_iter = line.chars().enumerate().filter_map(|(index, character)| {
                    if let Some(digit) = character.to_digit(10) {
                        Some(digit)
                    } else {
                        let sub_str = &line[index..];
                        DIGIT_STRS
                            .iter()
                            .enumerate()
                            .filter_map(|(digit, digit_str)| {
                                sub_str.starts_with(digit_str).then_some(digit as u32 + 1)
                            })
                            .next()
                    }
                });
                let first = digit_iter.next().expect("No digit on this line");
                let last = digit_iter.last().unwrap_or(first);
                first * 10 + last
            })
            .sum::<u32>();

        Some(result.to_string())
    }
}

const TEST_DATA_1: AoC2023<1, &str> = AoC2023::<1, &str> {
    input: "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
};

#[test]
fn test_part_one() {
    let result = TEST_DATA_1.part_one();
    println!("{:?}", result);
    assert_eq!(result, Some(String::from("142")));
}

const TEST_DATA_2: AoC2023<1, &str> = AoC2023::<1, &str> {
    input: "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
};

#[test]
fn test_part_two() {
    let result = TEST_DATA_2.part_two();
    println!("{:?}", result);
    assert_eq!(result, Some(String::from("281")));
}

const DIGIT_STRS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let d1 = AoC2023::<1, &str>::default();
    let r1 = d1.eval();
    println!("{:?}", r1);
}
