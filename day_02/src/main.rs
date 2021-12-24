use std::{num::ParseIntError, str::FromStr};

fn main() {
    println!(
        "Your final position is: {}",
        day_two_part_one("day_two_data.txt").unwrap()
    );

    println!(
        "Your final position is: {}",
        day_two_part_two("day_two_data.txt").unwrap()
    );
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

#[cfg(test)]
mod tests {
    use crate::{day_two_part_one, day_two_part_two};

    #[test]
    fn day_two_part_one_example() {
        let result = day_two_part_one("day_two_example.txt").unwrap();
        assert_eq!(result, 150);
    }

    #[test]
    fn day_two_part_two_example() {
        let result = day_two_part_two("day_two_example.txt").unwrap();
        assert_eq!(result, 900);
    }
}
