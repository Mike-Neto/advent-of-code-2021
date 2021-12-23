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

    println!(
        "The power consumption of the submarine is: {}",
        day_3_part_2("./data/day_3_data.txt").unwrap()
    );

    println!(
        "Your final score will be: {}",
        day_4_part_1("./data/day_4_data.txt").unwrap()
    );

    println!(
        "The last winning board final score would be: {}",
        day_4_part_2("./data/day_4_data.txt").unwrap()
    );

    println!(
        "There are {} points that at least two lines overlap",
        day_5_part_1("./data/day_5_data.txt").unwrap()
    );

    println!(
        "There are {} points that at least two lines overlap",
        day_5_part_2("./data/day_5_data.txt").unwrap()
    );

    println!(
        "There should be {} lanternfish be after 80 days",
        day_6_part_1("./data/day_6_data.txt", 80).unwrap()
    );

    println!(
        "There should be {} lanternfish be after 256 days",
        day_6_part_2("./data/day_6_data.txt", 256).unwrap()
    );
}

fn day_6_part_2(path: &str, days: u32) -> Result<u64, std::io::Error> {
    let file = std::fs::read_to_string(path)?;
    let values: Vec<&str> = file.split('\n').filter(|s| !s.is_empty()).collect();

    let mut cache = vec![0usize; 9];
    for val in 0..8i8 {
        let total_fish = calc_fish(days, vec![val]);
        cache[val as usize] = total_fish;
    }

    let sum = values[0]
        .split(',')
        .filter_map(|v| v.parse().ok())
        .map(|fish: i8| cache[fish as usize])
        .fold(0usize, |mut sum, fish| {
            sum += fish;
            sum
        });

    Ok(sum as u64)
}

fn calc_fish(days: u32, initial_fish: Vec<i8>) -> usize {
    println!("total fishes for {:?} over {} days", initial_fish, days);
    let mut current_fish: Vec<i8> = initial_fish;
    for day in 1..=days {
        println!("day {}", day);
        let mut new_fish = 0;
        for fish in current_fish.iter_mut() {
            *fish -= 1;
            if *fish == -1 {
                *fish = 6;
                new_fish += 1;
            }
        }
        current_fish.resize(current_fish.len() + new_fish, 8);
    }

    current_fish.len()
}

fn day_6_part_1(path: &str, days: u32) -> Result<u64, std::io::Error> {
    let file = std::fs::read_to_string(path)?;
    let values: Vec<&str> = file.split('\n').filter(|s| !s.is_empty()).collect();
    let initial_fish: Vec<i8> = values[0]
        .split(',')
        .filter_map(|v| v.parse().ok())
        .collect();

    let mut current_fish: Vec<i8> = initial_fish;
    for _ in 1..=days {
        // Simulate the day
        let mut new_fish = 0;
        for fish in current_fish.iter_mut() {
            *fish -= 1;
            if *fish == -1 {
                *fish = 6;
                new_fish += 1;
            }
        }
        current_fish.resize(current_fish.len() + new_fish, 8);
    }

    Ok(current_fish.len() as u64)
}

fn day_5_part_2(path: &str) -> Result<u64, std::io::Error> {
    let file = std::fs::read_to_string(path)?;
    let lines: Vec<Vec<Vec<u32>>> = file
        .split('\n')
        .filter_map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(
                    s.split("->")
                        .map(|points| {
                            points
                                .trim()
                                .split(',')
                                .filter_map(|coord| coord.parse().ok())
                                .collect()
                        })
                        .collect(),
                )
            }
        })
        .collect();

    let max_point = lines.iter().flatten().flatten().max().unwrap() + 1;

    let mut overlap_grid = vec![vec![0u32; max_point as usize]; max_point as usize];

    for line in lines {
        let a = &line[0];
        let b = &line[1];

        let same_x = a[0] != b[0];
        let same_y = a[1] != b[1];
        let is_diagonal = same_x && same_y;
        if !is_diagonal {
            if same_x {
                for x in a[0]..=b[0] {
                    let y = a[1] as usize;
                    overlap_grid[y][x as usize] += 1;
                }

                // TODO find a way to not repeat both ways
                // TODO Add code that predicts direction instead for iterating both ranges
                for x in b[0]..=a[0] {
                    let y = a[1] as usize;
                    overlap_grid[y][x as usize] += 1;
                }
            }

            if same_y {
                for y in a[1]..=b[1] {
                    let x = a[0] as usize;
                    overlap_grid[y as usize][x] += 1;
                }

                // TODO find a way to not repeat both ways
                for y in b[1]..=a[1] {
                    let x = a[0] as usize;
                    overlap_grid[y as usize][x] += 1;
                }
            }
        } else {
            let (x_from, _x_to) = if a[0] < b[0] { (a, b) } else { (b, a) };
            let (y_from, y_to) = if a[1] < b[1] { (a, b) } else { (b, a) };
            for (index, y) in (y_from[1]..=y_to[1]).enumerate() {
                let x = if y_from[0] > y_to[0] {
                    y_from[0] as usize - index
                } else {
                    x_from[0] as usize + index
                };
                overlap_grid[y as usize][x] += 1;
            }
        }
    }

    let total = overlap_grid.iter().flatten().fold(0u32, |mut sum, &value| {
        if value >= 2 {
            sum += 1
        }
        sum
    });

    Ok(total as u64)
}

fn day_5_part_1(path: &str) -> Result<u64, std::io::Error> {
    let file = std::fs::read_to_string(path)?;
    let lines: Vec<Vec<Vec<u32>>> = file
        .split('\n')
        .filter_map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(
                    s.split("->")
                        .map(|points| {
                            points
                                .trim()
                                .split(',')
                                .filter_map(|coord| coord.parse().ok())
                                .collect()
                        })
                        .collect(),
                )
            }
        })
        .collect();

    let max_point = lines.iter().flatten().flatten().max().unwrap() + 1;

    let mut overlap_grid = vec![vec![0u32; max_point as usize]; max_point as usize];

    for line in lines {
        let a = &line[0];
        let b = &line[1];

        let same_x = a[0] != b[0];
        let same_y = a[1] != b[1];
        let is_diagonal = same_x && same_y;
        if !is_diagonal {
            if same_x {
                for x in a[0]..=b[0] {
                    let y = a[1] as usize;
                    overlap_grid[y][x as usize] += 1;
                }

                // TODO find a way to not repeat both ways
                for x in b[0]..=a[0] {
                    let y = a[1] as usize;
                    overlap_grid[y][x as usize] += 1;
                }
            }

            if same_y {
                for y in a[1]..=b[1] {
                    let x = a[0] as usize;
                    overlap_grid[y as usize][x] += 1;
                }

                // TODO find a way to not repeat both ways
                for y in b[1]..=a[1] {
                    let x = a[0] as usize;
                    overlap_grid[y as usize][x] += 1;
                }
            }
        }
    }

    let total = overlap_grid.iter().flatten().fold(0u32, |mut sum, &value| {
        if value >= 2 {
            sum += 1
        }
        sum
    });

    Ok(total as u64)
}

fn day_4_part_2(path: &str) -> Result<u64, std::io::Error> {
    let file = std::fs::read_to_string(path)?;
    let values: Vec<&str> = file.split('\n').filter(|s| !s.is_empty()).collect();
    let draw_numbers: Vec<u32> = values[0]
        .split(',')
        .filter_map(|v| v.parse().ok())
        .collect();

    let boards: Vec<Vec<Vec<u32>>> = values
        .iter()
        .skip(1)
        .chunks()
        .map(|[a, b, c, d, e]| {
            vec![
                a.split(' ').filter_map(|v| v.parse().ok()).collect(),
                b.split(' ').filter_map(|v| v.parse().ok()).collect(),
                c.split(' ').filter_map(|v| v.parse().ok()).collect(),
                d.split(' ').filter_map(|v| v.parse().ok()).collect(),
                e.split(' ').filter_map(|v| v.parse().ok()).collect(),
            ]
        })
        .collect();

    let mut incomplete_boards: Vec<&Vec<Vec<u32>>> = boards.iter().collect();
    for i in 0..(draw_numbers.len() - 1) {
        let numbers = &draw_numbers[0..i];
        let mut removed = 0usize;
        for (index, board) in incomplete_boards.clone().iter().enumerate() {
            let bingo = calculate_bingo(numbers, board);
            if let Some(score) = bingo {
                if incomplete_boards.len() == 1 {
                    return Ok(score as u64);
                } else {
                    incomplete_boards.remove(index - removed);
                    removed += 1;
                }
            }
        }
    }

    Ok(0)
}

fn day_4_part_1(path: &str) -> Result<u64, std::io::Error> {
    let file = std::fs::read_to_string(path)?;
    let values: Vec<&str> = file.split('\n').filter(|s| !s.is_empty()).collect();
    let draw_numbers: Vec<u32> = values[0]
        .split(',')
        .filter_map(|v| v.parse().ok())
        .collect();

    let boards: Vec<Vec<Vec<u32>>> = values
        .iter()
        .skip(1)
        .chunks()
        .map(|[a, b, c, d, e]| {
            vec![
                a.split(' ').filter_map(|v| v.parse().ok()).collect(),
                b.split(' ').filter_map(|v| v.parse().ok()).collect(),
                c.split(' ').filter_map(|v| v.parse().ok()).collect(),
                d.split(' ').filter_map(|v| v.parse().ok()).collect(),
                e.split(' ').filter_map(|v| v.parse().ok()).collect(),
            ]
        })
        .collect();

    for i in 0..(draw_numbers.len() - 1) {
        let numbers = &draw_numbers[0..i];
        for board in &boards {
            let bingo = calculate_bingo(numbers, board);
            if let Some(score) = bingo {
                return Ok(score as u64);
            }
        }
    }

    Ok(0)
}

fn calculate_bingo(numbers: &[u32], board: &[Vec<u32>]) -> Option<u32> {
    // Find if has bingo
    let matching_matrix: Vec<Vec<bool>> = board
        .iter()
        .map(|row| {
            row.iter()
                .map(|value| numbers.iter().any(|v| v == value))
                .collect()
        })
        .collect();

    let is_any_row_complete: bool = matching_matrix
        .iter()
        .any(|row| row.iter().all(|&value| value));

    let is_any_col_complete: bool = matching_matrix[0]
        .iter()
        .enumerate()
        .map(|(index, _)| matching_matrix.iter().map(move |row| &row[index]))
        .any(|mut col| col.all(|&value| value));

    if is_any_row_complete || is_any_col_complete {
        let unmarked_matrix: Vec<Vec<u32>> = matching_matrix
            .iter()
            .enumerate()
            .map(|(row_index, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(col_index, &value)| {
                        if !value {
                            board[row_index][col_index]
                        } else {
                            0
                        }
                    })
                    .collect()
            })
            .collect();
        let sum: u32 = unmarked_matrix.iter().fold(0u32, |mut sum, row| {
            sum += row.iter().fold(0u32, |mut sum, value| {
                sum += value;
                sum
            });
            sum
        });
        Some(sum * numbers.last().unwrap())
    } else {
        None
    }
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

fn day_3_part_2(path: &str) -> Result<u64, std::io::Error> {
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

    let mut current_bit = 0usize;
    let mut current_values: Vec<&Vec<&str>> = values.iter().collect();
    while current_values.len() != 1 && !current_values.is_empty() {
        current_values = filter_values(current_values, current_bit, true);
        current_bit += 1;
    }
    let oxygen = bits_to_u64(current_values[0]);

    let mut current_bit = 0usize;
    let mut current_values: Vec<&Vec<&str>> = values.iter().collect();
    while current_values.len() != 1 && !current_values.is_empty() {
        current_values = filter_values(current_values, current_bit, false);
        current_bit += 1;
    }
    let co2 = bits_to_u64(current_values[0]);

    Ok(oxygen * co2)
}

fn bits_to_u64(chars: &[&str]) -> u64 {
    let mut result = 0u64;
    for (index, &bit) in chars.iter().enumerate() {
        if bit == "1" {
            let shift = (chars.len() - 1) - index;
            let value: u64 = 1 << shift;
            result |= value;
        }
    }
    result
}

fn filter_values<'a>(
    current_values: Vec<&'a Vec<&str>>,
    current_bit: usize,
    is_oxygen_generator_rating: bool,
) -> Vec<&'a Vec<&'a str>> {
    let (ones, zeros) =
        current_values
            .iter()
            .map(|v| v[current_bit])
            .fold((0, 0), |(mut ones, mut zeros), v| {
                if v == "1" {
                    ones += 1;
                } else {
                    zeros += 1;
                }

                (ones, zeros)
            });

    let filtered_values: Vec<&Vec<&str>> = current_values
        .into_iter()
        .filter(|v| {
            if is_oxygen_generator_rating {
                if ones >= zeros {
                    v[current_bit] == "1"
                } else {
                    v[current_bit] == "0"
                }
            } else if zeros > ones {
                v[current_bit] == "1"
            } else {
                v[current_bit] == "0"
            }
        })
        .collect();

    filtered_values
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
        day_3_part_1, day_3_part_2, day_4_part_1, day_4_part_2, day_5_part_1, day_5_part_2,
        day_6_part_1, day_one_part_one, day_one_part_two, day_two_part_one, day_two_part_two,
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

    #[test]
    fn day_3_part_2_example() {
        let result = day_3_part_2("./data/day_3_example.txt").unwrap();
        assert_eq!(result, 230);
    }

    #[test]
    fn day_4_part_1_example() {
        let result = day_4_part_1("./data/day_4_example.txt").unwrap();
        assert_eq!(result, 4512);
    }

    #[test]
    fn day_4_part_2_example() {
        let result = day_4_part_2("./data/day_4_example.txt").unwrap();
        assert_eq!(result, 1924);
    }

    #[test]
    fn day_5_part_1_example() {
        let result = day_5_part_1("./data/day_5_example.txt").unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn day_5_part_2_example() {
        let result = day_5_part_2("./data/day_5_example.txt").unwrap();
        assert_eq!(result, 12);
    }

    #[test]
    fn day_6_part_1_example() {
        let result = day_6_part_1("./data/day_6_example.txt", 18).unwrap();
        assert_eq!(result, 26);
        let result = day_6_part_1("./data/day_6_example.txt", 80).unwrap();
        assert_eq!(result, 5934);
    }
    /*
    THIS takes to long to run in tests
    #[test]
    fn day_6_part_2_example() {
        let result = day_6_part_2("./data/day_6_example.txt", 256).unwrap();
        assert_eq!(result, 26984457539);
    }
     */
}
