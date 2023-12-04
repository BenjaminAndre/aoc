use crate::aoc::*;

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
