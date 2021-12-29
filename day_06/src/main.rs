fn main() {
    println!(
        "There should be {} lanternfish be after 80 days",
        simulate_fish("day_6_data.txt", 80).unwrap()
    );

    println!(
        "There should be {} lanternfish be after 256 days",
        simulate_fish("day_6_data.txt", 256).unwrap()
    );
}

fn simulate_one_day(timers: &mut [usize; 9]) {
    timers.rotate_left(1);
    timers[6] += timers[8];
}

fn simulate_fish(path: &str, days: u32) -> Result<usize, std::io::Error> {
    let file = std::fs::read_to_string(path)?;
    let values: Vec<&str> = file.split('\n').filter(|s| !s.is_empty()).collect();
    let mut timers = values[0]
        .split(',')
        .filter_map(|v| v.parse::<usize>().ok())
        .fold([0_usize; 9], |mut map, timer| {
            if let Some(v) = map.get_mut(timer) {
                *v += 1;
            }
            map
        });

    for _day in 1..=days {
        simulate_one_day(&mut timers);
    }

    Ok(timers.iter().sum())
}

#[cfg(test)]
mod tests {
    use crate::simulate_fish;

    #[test]
    fn simulate_fish_example() {
        let result = simulate_fish("day_6_example.txt", 18).unwrap();
        assert_eq!(result, 26);
        let result = simulate_fish("day_6_example.txt", 80).unwrap();
        assert_eq!(result, 5934);
        let result = simulate_fish("day_6_example.txt", 256).unwrap();
        assert_eq!(result, 26984457539);
    }
}
