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
