fn main() {
    println!(
        "There are {} points that at least two lines overlap",
        day_5_part_1("day_5_data.txt").unwrap()
    );

    println!(
        "There are {} points that at least two lines overlap",
        day_5_part_2("day_5_data.txt").unwrap()
    );
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

    let mut overlap_grid = vec![vec![0_u32; max_point as usize]; max_point as usize];

    for line in lines {
        let a = &line[0];
        let b = &line[1];

        let same_x = a[0] != b[0];
        let same_y = a[1] != b[1];
        let is_diagonal = same_x && same_y;
        if is_diagonal {
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
        } else {
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
        }
    }

    let total = overlap_grid
        .iter()
        .flatten()
        .fold(0_u32, |mut sum, &value| {
            if value >= 2 {
                sum += 1;
            }
            sum
        });

    Ok(u64::from(total))
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

    let mut overlap_grid = vec![vec![0_u32; max_point as usize]; max_point as usize];

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

    let total = overlap_grid
        .iter()
        .flatten()
        .fold(0_u32, |mut sum, &value| {
            if value >= 2 {
                sum += 1;
            }
            sum
        });

    Ok(u64::from(total))
}

#[cfg(test)]
mod tests {
    use crate::{day_5_part_1, day_5_part_2};
    #[test]
    fn day_5_part_1_example() {
        let result = day_5_part_1("day_5_example.txt").unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn day_5_part_2_example() {
        let result = day_5_part_2("day_5_example.txt").unwrap();
        assert_eq!(result, 12);
    }
}
