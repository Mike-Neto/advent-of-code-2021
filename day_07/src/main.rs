fn main() {
    println!(
        "The least amount of fuel we can spend to align all crabs is: {}",
        align_crabs("data.txt").unwrap()
    );
}

fn align_crabs(path: &str) -> Result<i32, std::io::Error> {
    let file = std::fs::read_to_string(path)?;
    let values: Vec<&str> = file.split('\n').filter(|s| !s.is_empty()).collect();
    let initial_crabs: Vec<u32> = values[0]
        .split(',')
        .filter_map(|v| v.parse().ok())
        .collect();

    let fuel_spent: Vec<i32> = (0..initial_crabs.len())
        .into_iter()
        .map(|index| {
            initial_crabs.iter().fold(0i32, |mut sum, &crab_offset| {
                let position = index + 1;
                sum += ((position - crab_offset as usize) as i32).abs();
                sum
            })
        })
        .collect();

    Ok(fuel_spent.into_iter().min().unwrap())
}

#[cfg(test)]
mod tests {
    use crate::align_crabs;

    #[test]
    fn align_crabs_example() {
        let result = align_crabs("example.txt").unwrap();
        assert_eq!(result, 37);
    }
    #[test]
    fn align_crabs_attempt_1() {
        // Answer was too low
        let result = align_crabs("data.txt").unwrap();
        assert_ne!(result, 25647);
        assert!(result > 25647);
    }
}