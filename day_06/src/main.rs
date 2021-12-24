fn main() {
    println!(
        "There should be {} lanternfish be after 80 days",
        day_6_part_1("day_6_data.txt", 80).unwrap()
    );

    println!(
        "There should be {} lanternfish be after 256 days",
        day_6_part_2("day_6_data.txt", 256).unwrap()
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
    let mut current_fish: Vec<i8> = initial_fish;
    for _day in 1..=days {
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

#[cfg(test)]
mod tests {
    use crate::{day_6_part_1, day_6_part_2};

    #[test]
    fn day_6_part_1_example() {
        let result = day_6_part_1("day_6_example.txt", 18).unwrap();
        assert_eq!(result, 26);
        let result = day_6_part_1("day_6_example.txt", 80).unwrap();
        assert_eq!(result, 5934);
    }

    #[test]
    fn day_6_part_2_example() {
        let result = day_6_part_2("day_6_example.txt", 256).unwrap();
        assert_eq!(result, 26984457539);
    }
}
