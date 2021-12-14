use advent_of_code_2021::detect_increases;
use bit_vec::BitVec;
use itermore::IterMore;
use std::{num::ParseIntError, str::FromStr};

fn main() {
    println!(
        "there are {} measurements that are larger than the previous measurement",
        day_one_part_one("./data/day_one_data.txt").unwrap()
    );

    println!(
        "there are {} measurements that are larger than the previous measurement",
        day_one_part_two("./data/day_one_data.txt").unwrap()
    );

    println!(
        "Your final position is: {}",
        day_two_part_one("./data/day_two_data.txt").unwrap()
    );

    println!(
        "Your final position is: {}",
        day_two_part_two("./data/day_two_data.txt").unwrap()
    );

    println!(
        "The power consumption of the submarine is: {}",
        day_3_part_1("./data/day_3_data.txt").unwrap()
    );
}

fn day_3_part_1(path: &str) -> Result<u64, std::io::Error> {
    let file = std::fs::read_to_string(path)?;
    let values: Vec<Vec<&str>> = file
        .split('\n')
        .filter_map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s.split("").filter(|s| !s.is_empty()).collect())
            }
        })
        .collect();
    let total_data_rows = values.len();
    let total_data_cols = values[0].len();
    let mut counts = vec![0usize; total_data_cols];
    for v in values {
        for (index, &b) in v.iter().enumerate() {
            if "1" == b {
                counts[index] += 1;
            }
        }
    }

    let tipping_point = total_data_rows / 2;
    let gamma_bits: Vec<char> = counts
        .iter()
        .map(|&c| if c > tipping_point { '1' } else { '0' })
        .collect();

    let mut gamma = BitVec::from_elem(total_data_cols, false);
    for (index, &bit) in gamma_bits.iter().enumerate() {
        if bit == '1' {
            gamma.set(index, true);
        }
    }

    let mut gamma_value = 0u64;
    let mut epsilon_value = 0u64;
    for (index, bit) in gamma.iter().enumerate() {
        let shift = (total_data_cols - 1) - index;
        let value: u64 = 1 << shift;
        if bit {
            gamma_value |= value;
        } else {
            epsilon_value |= value;
        }
    }

    Ok(gamma_value * epsilon_value)
}

#[derive(Debug)]
enum Action {
    Forward(u32),
    Down(u32),
    Up(u32),
}

enum ParseActionError {
    Format(String),
    Action(String),
    Magnitude(String),
}

impl FromStr for Action {
    type Err = ParseActionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split(' ').collect();
        if v.len() != 2 {
            return Err(ParseActionError::Format(s.to_string()));
        }
        let action = v[0];
        let magnitude: &str = v[1];
        let magnitude: u32 = magnitude
            .parse()
            .map_err(|err: ParseIntError| ParseActionError::Magnitude(err.to_string()))?;

        if action == "forward" {
            Ok(Action::Forward(magnitude))
        } else if action == "down" {
            Ok(Action::Down(magnitude))
        } else if action == "up" {
            Ok(Action::Up(magnitude))
        } else {
            Err(ParseActionError::Action(action.to_string()))
        }
    }
}

fn day_two_part_one(path: &str) -> Result<i64, std::io::Error> {
    let values: Vec<Action> = std::fs::read_to_string(path)?
        .split('\n')
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut horizontal = 0i64;
    let mut depth = 0i64;
    for v in values {
        match v {
            Action::Down(magnitude) => {
                depth += magnitude as i64;
            }
            Action::Up(magnitude) => {
                depth -= magnitude as i64;
            }
            Action::Forward(magnitude) => {
                horizontal += magnitude as i64;
            }
        }
    }

    Ok(horizontal * depth)
}

fn day_two_part_two(path: &str) -> Result<i64, std::io::Error> {
    let values: Vec<Action> = std::fs::read_to_string(path)?
        .split('\n')
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut horizontal = 0i64;
    let mut depth = 0i64;
    let mut aim = 0i64;
    for v in values {
        match v {
            Action::Down(magnitude) => {
                aim += magnitude as i64;
            }
            Action::Up(magnitude) => {
                aim -= magnitude as i64;
            }
            Action::Forward(magnitude) => {
                horizontal += magnitude as i64;
                depth += aim * magnitude as i64;
            }
        }
    }

    Ok(horizontal * depth)
}

fn day_one_part_one(path: &str) -> Result<u64, std::io::Error> {
    let values: Vec<u32> = std::fs::read_to_string(path)?
        .split('\n')
        .filter_map(|s| s.parse().ok())
        .collect();

    Ok(detect_increases(&values))
}

fn day_one_part_two(path: &str) -> Result<u64, std::io::Error> {
    let values: Vec<u32> = std::fs::read_to_string(path)?
        .split('\n')
        .filter_map(|s| s.parse().ok())
        .windows()
        .map(|[x, y, z]: [u32; 3]| x + y + z)
        .collect();

    Ok(detect_increases(&values))
}

#[cfg(test)]
mod tests {
    use crate::{
        day_3_part_1, day_one_part_one, day_one_part_two, day_two_part_one, day_two_part_two,
    };

    #[test]
    fn day_one_part_one_example() {
        let result = day_one_part_one("./data/day_one_example.txt").unwrap();
        assert_eq!(result, 7);
    }

    #[test]
    fn day_one_part_two_example() {
        let result = day_one_part_two("./data/day_one_example.txt").unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn day_two_part_one_example() {
        let result = day_two_part_one("./data/day_two_example.txt").unwrap();
        assert_eq!(result, 150);
    }

    #[test]
    fn day_two_part_two_example() {
        let result = day_two_part_two("./data/day_two_example.txt").unwrap();
        assert_eq!(result, 900);
    }

    #[test]
    fn day_3_part_1_example() {
        let result = day_3_part_1("./data/day_3_example.txt").unwrap();
        assert_eq!(result, 198);
    }
}
