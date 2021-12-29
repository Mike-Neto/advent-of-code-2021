fn main() {
    println!(
        "The least amount of fuel we can spend to align all crabs is: {}",
        align_crabs("data.txt").unwrap()
    );

    println!(
        "The least amount of fuel we can spend to align all crabs is: {}",
        align_crabs_part_2("data.txt").unwrap()
    );
}

fn align_crabs(path: &str) -> Result<i64, std::io::Error> {
    let file = std::fs::read_to_string(path)?;
    let values: Vec<&str> = file.split('\n').filter(|s| !s.is_empty()).collect();
    let initial_crabs: Vec<i64> = values[0]
        .split(',')
        .filter_map(|v| v.parse().ok())
        .collect();

    let fuel_spent: Vec<i64> = (0..initial_crabs.len())
        .into_iter()
        .map(|index| {
            initial_crabs.iter().fold(0i64, |mut sum, &crab_offset| {
                let position = index + 1;
                let result = i64::try_from(position).unwrap() - crab_offset;
                sum += result.abs();
                sum
            })
        })
        .collect();

    Ok(fuel_spent.into_iter().min().unwrap())
}

fn align_crabs_part_2(path: &str) -> Result<i64, std::io::Error> {
    let file = std::fs::read_to_string(path)?;
    let values: Vec<&str> = file.split('\n').filter(|s| !s.is_empty()).collect();
    let initial_crabs: Vec<i64> = values[0]
        .split(',')
        .filter_map(|v| v.parse().ok())
        .collect();

    let fuel_spent: Vec<i64> = (0..initial_crabs.len())
        .into_iter()
        .map(|index| {
            initial_crabs.iter().fold(0i64, |mut sum, &crab_offset| {
                let position = index + 1;
                let result = i64::try_from(position).unwrap() - crab_offset;
                let distance = result.abs();
                sum += ((distance * distance) + distance) / 2;
                sum
            })
        })
        .collect();

    Ok(fuel_spent.into_iter().min().unwrap())
}

#[cfg(test)]
mod tests {
    use crate::{align_crabs, align_crabs_part_2};

    #[test]
    fn align_crabs_example() {
        let result = align_crabs("example.txt").unwrap();
        assert_eq!(result, 37);
    }

    #[test]
    fn align_crabs_attempt_1() {
        let result = align_crabs("data.txt").unwrap();
        assert_ne!(result, 25647);
        assert!(result > 25647);
    }

    #[test]
    fn align_crabs_part_2_example() {
        let result = align_crabs_part_2("example.txt").unwrap();
        assert_eq!(result, 168);
    }
}
