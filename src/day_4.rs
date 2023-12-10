use std::collections::BinaryHeap;
use std::{result, u32, usize};

use crate::aoc::*;

#[derive(Debug)]
struct Card {
    value: u32,
    winning_numbers: BinaryHeap<u32>,
    elf_numbers: Vec<u32>,
}

impl AoC2023<4, &str> {
    fn process_card(line_number: usize, line: &str) -> Card {
        let data = line.split_once(':').unwrap().1;

        let mut is_in_elf_data = false;
        let mut winning_numbers = BinaryHeap::<u32>::new();
        let mut elf_numbers = Vec::<u32>::new();

        let mut number_builder = String::new();

        data.chars().for_each(|c| {
            if c >= '0' && c <= '9' {
                number_builder.push(c);
            } else {
                match u32::from_str_radix(&number_builder, 10) {
                    Ok(value) => {
                        if is_in_elf_data {
                            elf_numbers.push(value);
                        } else {
                            winning_numbers.push(value);
                        }
                    }
                    Err(_) => (),
                }
                if c == '|' {
                    is_in_elf_data = true;
                }
                number_builder.clear();
            }
        });

        Card {
            value: u32::try_from(line_number).unwrap() + 1,
            winning_numbers,
            elf_numbers,
        }
    }

    fn preprocess(&self) -> Vec<Card> {
        self.input
            .lines()
            .enumerate()
            .map(|line_tuple| Self::process_card(line_tuple.0, &(line_tuple.1.to_owned() + " ")))
            .collect()
    }

    fn count_card(card: &Card) -> u32 {
        u32::try_from(
            card.elf_numbers
                .iter()
                .filter(|elf_number| {
                    card.winning_numbers
                        .iter()
                        .any(|w| w == elf_number.to_owned())
                })
                .count(),
        )
        .unwrap()
    }

    fn score_count(value: u32) -> u32 {
        match u32::try_from(value).unwrap().checked_sub(1) {
            Some(exp) => 2u32.pow(exp),
            None => 0,
        }
    }

    fn score_card(card: &Card) -> u32 {
        Self::score_count(Self::count_card(card))
    }
}

impl Day for AoC2023<4, &str> {
    fn default() -> Self {
        Self {
            input: include_str!("../data/input4.txt"),
        }
    }

    fn part_one(&self) -> Option<String> {
        let cards = self.preprocess();
        let sum = cards.iter().map(|card| Self::score_card(card)).sum::<u32>();
        Some(sum.to_string())
    }

    fn part_two(&self) -> Option<String> {
        let cards = self.preprocess();
        let counts: Vec<u32> = cards.iter().map(|card| Self::count_card(card)).collect();
        let mut card_counts = vec![1u32; counts.len()];
        for i in 0..counts.len() {
            if counts[i] == 0 {
                continue;
            }
            for offset in 1..counts[i] + 1 {
                let offsetted = (i as u32 + offset) as usize;
                if offsetted >= counts.len() {
                    break;
                }
                card_counts[offsetted] += card_counts[i];
            }
        }
        Some(card_counts.iter().sum::<u32>().to_string())
    }
}

#[test]
fn test_part_one() {
    const TEST_DATA_1: AoC2023<4, &str> = AoC2023::<4, &str> {
        input: "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    };
    let result = TEST_DATA_1.part_one();
    println!("{:?}", result);
    assert_eq!(result, Some(String::from("13")));
}

#[test]
fn test_part_two() {
    const TEST_DATA_1: AoC2023<4, &str> = AoC2023::<4, &str> {
        input: "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    };
    let result = TEST_DATA_1.part_two();
    println!("{:?}", result);
    assert_eq!(result, Some(String::from("30")));
}
