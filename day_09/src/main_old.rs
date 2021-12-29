fn main() {
    println!(
        "The sum of the risk levels of all low points is: {}",
        sum_risk_levels("data.txt").unwrap()
    );

    println!(
        "The product of the three largest basins is: {}",
        product_largest_basins("data.txt").unwrap()
    );
}

fn product_largest_basins(path: &str) -> Result<usize, std::io::Error> {
    let file = std::fs::read_to_string(path)?;

    let height_map: Vec<Vec<u8>> = file
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|line| line.split("").filter_map(|v| v.parse().ok()).collect())
        .collect();

    let max_row_index = height_map.len() - 1;
    let max_col_index = height_map[0].len() - 1;
    let mut basin_sizes: Vec<usize> = vec![];
    for (row_index, row) in height_map.iter().enumerate() {
        for (col_index, &value) in row.iter().enumerate() {
            let is_lower_than_up =
                row_index > 0 && height_map[row_index - 1][col_index] > value || row_index == 0;
            let is_lower_than_right = col_index < max_col_index
                && height_map[row_index][col_index + 1] > value
                || col_index == max_col_index;
            let is_lower_than_down = row_index < max_row_index
                && height_map[row_index + 1][col_index] > value
                || row_index == max_row_index;
            let is_lower_than_left =
                col_index > 0 && height_map[row_index][col_index - 1] > value || col_index == 0;

            if is_lower_than_down && is_lower_than_up && is_lower_than_right && is_lower_than_left {
                let mut has_found_tiles = true;
                let mut basin = vec![(row_index, col_index)];
                let mut distance = 1;
                while has_found_tiles {
                    has_found_tiles = false;
                    let start_row_index = row_index as i64 - distance as i64;
                    let end_row_index = row_index as i64 + distance as i64;
                    let start_col_index = col_index as i64 - distance as i64;
                    let end_col_index = col_index as i64 + distance as i64;

                    // TODO use iterator.chain() to concat all the iters
                    let top_iter = (start_row_index..=end_row_index)
                        .filter(|&row| row >= 0)
                        .map(|f| (f, start_col_index));
                    let bottom_iter = (start_row_index..=end_row_index)
                        .filter(|&row| row >= 0)
                        .map(|f| (f, end_col_index));

                    // TODO CHeck that each tile is adjacent to any value in the basin already

                    // TODO This checks area of square, only want to check perimeter
                    for (r, c) in top_iter.chain(bottom_iter) {
                        // Check if in basin
                        println!("checking {}, {}", r, c);
                        let is_basin = true;
                        if is_basin {
                            basin.push((r as usize, c as usize));
                            has_found_tiles = true;
                        }
                    }
                    distance += 1;
                }
                basin_sizes.push(basin.len());

                // SO I know i'm right there in the "middle" so go round and around in squares of increasing
                // distance to me until I have none, this should count all the squares in the basin
                // Need to think about occlusion, maybe add a second matrix that tracks that
            }
        }
    }

    basin_sizes.sort_unstable();
    let product = basin_sizes.iter().rev().take(3).product();

    Ok(product)
}

fn sum_risk_levels(path: &str) -> Result<usize, std::io::Error> {
    let file = std::fs::read_to_string(path)?;

    let height_map: Vec<Vec<u8>> = file
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|line| line.split("").filter_map(|v| v.parse().ok()).collect())
        .collect();

    let max_row_index = height_map.len() - 1;
    let max_col_index = height_map[0].len() - 1;
    let mut low_points: Vec<u8> = vec![];
    for (row_index, row) in height_map.iter().enumerate() {
        for (col_index, &value) in row.iter().enumerate() {
            let is_lower_than_up =
                row_index > 0 && height_map[row_index - 1][col_index] > value || row_index == 0;
            let is_lower_than_right = col_index < max_col_index
                && height_map[row_index][col_index + 1] > value
                || col_index == max_col_index;
            let is_lower_than_down = row_index < max_row_index
                && height_map[row_index + 1][col_index] > value
                || row_index == max_row_index;
            let is_lower_than_left =
                col_index > 0 && height_map[row_index][col_index - 1] > value || col_index == 0;

            if is_lower_than_down && is_lower_than_up && is_lower_than_right && is_lower_than_left {
                low_points.push(value);
            }
        }
    }

    let sum = low_points
        .iter()
        .map(|point| point + 1)
        .fold(0_usize, |mut sum, value| {
            sum += value as usize;
            sum
        });

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use crate::{product_largest_basins, sum_risk_levels};

    #[test]
    fn sum_risk_levels_example() {
        let result = sum_risk_levels("example.txt").unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn product_largest_basins_example() {
        let result = product_largest_basins("example.txt").unwrap();
        assert_eq!(result, 1134);
    }
}
