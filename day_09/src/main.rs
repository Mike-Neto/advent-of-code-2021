fn main() {
    println!(
        "The sum of the risk levels of all low points is: {}",
        sum_risk_levels("data.txt").unwrap()
    );
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
        .fold(0usize, |mut sum, value| {
            sum += value as usize;
            sum
        });

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use crate::sum_risk_levels;

    #[test]
    fn sum_risk_levels_example() {
        let result = sum_risk_levels("example.txt").unwrap();
        assert_eq!(result, 15);
    }
}
