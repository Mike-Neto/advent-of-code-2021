use advent_of_code_2021::detect_increases;
use itermore::IterMore;

fn main() {
    println!(
        "there are {} measurements that are larger than the previous measurement",
        day_one_part_one("./data/day_one_data.txt").unwrap()
    );

    println!(
        "there are {} measurements that are larger than the previous measurement",
        day_one_part_two("./data/day_one_data.txt").unwrap()
    );
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
    use crate::{day_one_part_one, day_one_part_two};

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
}
