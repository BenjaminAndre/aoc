use std::cmp::max;

use crate::aoc::*;

#[derive(Default, Debug)]
struct Picks(u32, u32, u32);

impl Picks {
    fn respects(&self, r: u32, g: u32, b: u32) -> bool {
        self.0 <= r && self.1 <= g && self.2 <= b
    }
}

#[derive(Default, Debug)]
struct Game {
    id: u32,
    r: u32,
    g: u32,
    b: u32,
    picks: Vec<Picks>,
}

impl Game {
    fn score(&self) -> u32 {
        if self
            .picks
            .iter()
            .any(|pick| !pick.respects(self.r, self.g, self.b))
        {
            0
        } else {
            self.id
        }
    }

    fn minimum_powers(&self) -> u32 {
        let min_r = self.picks.iter().fold(0, |acc, pick| max(acc, pick.0));
        let min_g = self.picks.iter().fold(0, |acc, pick| max(acc, pick.1));
        let min_b = self.picks.iter().fold(0, |acc, pick| max(acc, pick.2));
        min_r * min_g * min_b
    }
}

impl AoC2023<2, &str> {
    fn parse_day(&self) -> Option<Vec<Game>> {
        let mut games: Vec<Game> = vec![];
        for line in self.input.lines() {
            let line_str = line.to_string();
            let (id_word, game_text) = line_str.split_once(':').unwrap();
            let mut game = Game::default();
            game.id = id_word.split_once(' ').unwrap().1.parse::<u32>().unwrap();
            game.r = 12;
            game.g = 13;
            game.b = 14;
            for pick in game_text.split(';') {
                let mut pick_result = Picks::default();
                for color in pick.split(',') {
                    let (dice, color_name) = color.trim().split_once(' ').unwrap();
                    let dn = dice.trim().parse::<u32>().unwrap();
                    match color_name.trim().chars().next().unwrap() {
                        'r' => pick_result.0 += dn,
                        'g' => pick_result.1 += dn,
                        'b' => pick_result.2 += dn,
                        _ => {
                            println!("Failed to parse the input on line {:?}", line);
                            return None;
                        }
                    }
                }
                game.picks.push(pick_result);
            }
            games.push(game);
        }
        Some(games)
    }
}

impl Day for AoC2023<2, &str> {
    fn default() -> Self {
        Self {
            input: include_str!("../data/input2.txt"),
        }
    }

    fn part_one(&self) -> Option<String> {
        Some(
            self.parse_day()
                .unwrap()
                .iter()
                .map(|game| game.score())
                .sum::<u32>()
                .to_string(),
        )
    }

    fn part_two(&self) -> Option<String> {
        Some(
            self.parse_day()
                .unwrap()
                .iter()
                .map(|game| game.minimum_powers())
                //                .inspect(|pow| println!("{:?}", pow))
                .sum::<u32>()
                .to_string(),
        )
    }
}

#[test]
fn test_part_one() {
    const TEST_DATA_1: AoC2023<2, &str> = AoC2023::<2, &str> {
        input: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    };
    let result = TEST_DATA_1.part_one();
    println!("{:?}", result);
    assert_eq!(result, Some(String::from("8")));
}

#[test]
fn test_part_two() {
    const TEST_DATA_1: AoC2023<2, &str> = AoC2023::<2, &str> {
        input: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    };
    let result = TEST_DATA_1.part_two();
    println!("{:?}", result);
    assert_eq!(result, Some(String::from("2286")));
}
