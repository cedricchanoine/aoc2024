use regex::{Captures, Regex};
use std::{num::ParseIntError, sync::LazyLock};

const INPUT: &str = include_str!("../d3_data");
mod part1 {
    use super::*;

    static REGEX1: LazyLock<Regex> = {
        LazyLock::new(|| {
            let pattern = r"mul\((\d+),(\d+)\)";
            Regex::new(pattern).expect("can't compile the regex")
        })
    };

    pub fn run() -> u32 {
        REGEX1
            .captures_iter(INPUT)
            .filter(|c| c.len() == 3)
            .filter_map(|c| match (c[1].parse::<u32>(), c[2].parse::<u32>()) {
                (Ok(a), Ok(b)) => Some((a, b)),
                _ => None,
            })
            .map(|(a, b)| a * b)
            .sum()
    }
}

mod part2 {
    use super::*;

    static REGEX2: LazyLock<Regex> = {
        LazyLock::new(|| {
            let pattern = r"(?x)
                   (?:don't\(\))
                   |
                   (?:do\(\))
                   |
                   mul\((\d+),(\d+)\)
               ";
            Regex::new(pattern).expect("can't compile the regex")
        })
    };
    #[derive(Debug)]
    enum Token {
        Mul(u32, u32),
        Do,
        Dont,
    }

    impl<'h> TryFrom<Captures<'h>> for Token {
        type Error = ParseIntError;
        fn try_from(value: Captures<'h>) -> Result<Self, Self::Error> {
            match &value[0] {
                "don't()" => Ok(Self::Dont),
                "do()" => Ok(Self::Do),
                _ => {
                    assert_eq!(value.len(), 3, "wtf {:?}", value);
                    let a = value[1].parse::<u32>()?;
                    let b = value[2].parse::<u32>()?;
                    Ok(Self::Mul(a, b))
                }
            }
        }
    }

    struct InstructionProcessIt<I: Iterator<Item = Token>> {
        processing: bool,
        it: I,
    }

    impl<I> Iterator for InstructionProcessIt<I>
    where
        I: Iterator<Item = Token>,
    {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            let value = self.it.next()?;
            match value {
                Token::Mul(a, b) => {
                    if self.processing {
                        Some(a * b)
                    } else {
                        Some(0)
                    }
                }
                Token::Do => {
                    self.processing = true;
                    Some(0)
                }
                Token::Dont => {
                    self.processing = false;
                    Some(0)
                }
            }
        }
    }

    pub fn run() -> u32 {
        let tokens_it = REGEX2
            .captures_iter(INPUT)
            .filter_map(|c| Token::try_from(c).ok());

        let process_instructions = InstructionProcessIt {
            processing: true,
            it: tokens_it,
        };
        process_instructions.sum()
    }
}

fn main() {
    let p1 = part1::run();
    let p2 = part2::run();
    println!("{p1} {p2}");
}
