use core::num;
use std::result;

use crate::aoc::*;

#[derive(Debug)]
struct SymbolCoords {
    is_gear: bool,
    position: (i32, i32),
}

#[derive(Debug)]
struct NumberCoords {
    value: u32,
    start_corner: (i32, i32),
    end_corner: (i32, i32),
}

#[derive(Debug)]
struct PreprocessedProblem {
    symbols: Vec<SymbolCoords>,
    numbers: Vec<NumberCoords>,
}

impl AoC2023<3, &str> {
    fn preprocess(&self) -> PreprocessedProblem {
        let mut result = PreprocessedProblem {
            symbols: Vec::<SymbolCoords>::new(),
            numbers: Vec::<NumberCoords>::new(),
        };

        self.input
            .lines()
            .enumerate()
            .for_each(|(line_number, line)| {
                let j = i32::try_from(line_number).unwrap();
                let mut number_builder = String::new();
                line.chars().enumerate().for_each(|(char_number, c)| {
                    let i = i32::try_from(char_number).unwrap();
                    if c >= '0' && c <= '9' {
                        number_builder.push(c);
                    } else {
                        match number_builder.parse::<u32>() {
                            Ok(number) => result.numbers.push(NumberCoords {
                                value: number,
                                start_corner: (
                                    i - i32::try_from(number_builder.len()).unwrap() - 1,
                                    j - 1,
                                ),
                                end_corner: (i, j + 1),
                            }),
                            Err(_e) => (),
                        }
                        number_builder = String::new();
                        // try parse the string
                        if c != '.' {
                            result.symbols.push(SymbolCoords {
                                is_gear: c == '*',
                                position: (i, j),
                            });
                        }
                    }
                })
            });
        result
    }
}

impl Day for AoC2023<3, &str> {
    fn default() -> Self {
        Self {
            input: include_str!("../data/input3.txt"),
        }
    }

    fn part_one(&self) -> Option<String> {
        let preprocessed = self.preprocess();
        let result = preprocessed
            .numbers
            .iter()
            .filter(|number_coords| {
                preprocessed.symbols.iter().any(|symbol| {
                    symbol.position.0 >= number_coords.start_corner.0
                        && symbol.position.0 <= number_coords.end_corner.0
                        && symbol.position.1 >= number_coords.start_corner.1
                        && symbol.position.1 <= number_coords.end_corner.1
                })
            })
            .map(|number_coords| number_coords.value)
            .sum::<u32>();
        Some(result.to_string())
    }

    fn part_two(&self) -> Option<String> {
        let preprocessed = self.preprocess();
        let result = preprocessed
            .symbols
            .iter()
            .map(|symbol| {
                let numbers: Vec<&NumberCoords> = preprocessed
                    .numbers
                    .iter()
                    .filter(|number_coords| {
                        symbol.position.0 >= number_coords.start_corner.0
                            && symbol.position.0 <= number_coords.end_corner.0
                            && symbol.position.1 >= number_coords.start_corner.1
                            && symbol.position.1 <= number_coords.end_corner.1
                    })
                    .collect();
                if numbers.len() == 2 && symbol.is_gear {
                    numbers.get(0).unwrap().value * numbers.get(1).unwrap().value
                } else {
                    0u32
                }
            })
            .sum::<u32>();
        Some(result.to_string())
    }
}

#[test]
fn test_part_one() {
    const TEST_DATA_1: AoC2023<3, &str> = AoC2023::<3, &str> {
        input: "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
    };
    let result = TEST_DATA_1.part_one();
    println!("{:?}", result);
    assert_eq!(result, Some(String::from("4361")));
}

#[test]
fn test_part_two() {
    const TEST_DATA_1: AoC2023<3, &str> = AoC2023::<3, &str> {
        input: "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
    };
    let result = TEST_DATA_1.part_two();
    println!("{:?}", result);
    assert_eq!(result, Some(String::from("467835")));
}
