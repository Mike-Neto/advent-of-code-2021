use itermore::IterMore;

fn main() {
    println!(
        "Your final score will be: {}",
        day_4_part_1("day_4_data.txt").unwrap()
    );

    println!(
        "The last winning board final score would be: {}",
        day_4_part_2("day_4_data.txt").unwrap()
    );
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
        let mut removed = 0_usize;
        for (index, board) in incomplete_boards.clone().iter().enumerate() {
            let bingo = calculate_bingo(numbers, board);
            if let Some(score) = bingo {
                if incomplete_boards.len() == 1 {
                    return Ok(u64::from(score));
                }
                incomplete_boards.remove(index - removed);
                removed += 1;
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
                return Ok(u64::from(score));
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
                        if value {
                            0
                        } else {
                            board[row_index][col_index]
                        }
                    })
                    .collect()
            })
            .collect();
        let sum: u32 = unmarked_matrix.iter().fold(0_u32, |mut sum, row| {
            sum += row.iter().fold(0_u32, |mut sum, value| {
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

#[cfg(test)]
mod tests {
    use crate::{day_4_part_1, day_4_part_2};

    #[test]
    fn day_4_part_1_example() {
        let result = day_4_part_1("day_4_example.txt").unwrap();
        assert_eq!(result, 4512);
    }

    #[test]
    fn day_4_part_2_example() {
        let result = day_4_part_2("day_4_example.txt").unwrap();
        assert_eq!(result, 1924);
    }
}
