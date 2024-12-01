use std::{collections::HashMap, num::ParseIntError};

const INPUT: &str = include_str!("../d1_data");

struct ParsedInput {
    l_list: Vec<u64>,
    r_list: Vec<u64>,
}

impl ParsedInput {
    fn sort(&mut self) {
        self.l_list.sort();
        self.r_list.sort();
    }
}

#[derive(derive_more::Debug, derive_more::Display)]
enum ParsingError {
    #[display("'{}' could not be parsed as u64, cause : {}", num, error)]
    #[debug("'{}' could not be parsed as u64, cause : {}", num, error)]
    Parsing { error: ParseIntError, num: String },
    #[display("left list length is {left} vs {right} for right")]
    #[debug("left list length is {left} vs {right} for right")]
    LengthError { left: usize, right: usize },
}

impl TryFrom<&str> for ParsedInput {
    type Error = ParsingError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (mut l1, mut l2) = (Vec::new(), Vec::new());

        let mut push_left = true;
        for s in value.split_whitespace() {
            let number: u64 = s.parse().map_err(|error| {
                let num = s.to_string();
                ParsingError::Parsing { error, num }
            })?;
            if push_left {
                l1.push(number);
            } else {
                l2.push(number);
            }
            push_left = !push_left;
        }
        if l1.len() != l2.len() {
            return Err(ParsingError::LengthError {
                left: l1.len(),
                right: l2.len(),
            });
        }
        Ok(ParsedInput {
            l_list: l1,
            r_list: l2,
        })
    }
}

fn p1(parsed_input: &ParsedInput) -> u64 {
    let diffs_sum: u64 = parsed_input
        .l_list
        .iter()
        .zip(parsed_input.r_list.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();
    diffs_sum
}

fn p2(parsed_input: &ParsedInput) -> u64 {
    let mut map: HashMap<u64, u64> = HashMap::new();
    parsed_input.r_list.iter().for_each(|e| {
        *map.entry(*e).or_insert(0) += 1;
    });
    let sum = parsed_input
        .l_list
        .iter()
        .map(|e| e * *map.get(e).unwrap_or(&0))
        .sum();

    sum
}

fn main() -> Result<(), ParsingError> {
    let mut input = ParsedInput::try_from(INPUT)?;
    input.sort();
    let p1_res = p1(&input);
    let p2_res = p2(&input);
    println!("{p1_res} {p2_res}");
    Ok(())
}
