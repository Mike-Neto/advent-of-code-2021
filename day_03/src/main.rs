use bit_vec::BitVec;

fn main() {
    println!(
        "The power consumption of the submarine is: {}",
        day_3_part_1("day_3_data.txt").unwrap()
    );

    println!(
        "The power consumption of the submarine is: {}",
        day_3_part_2("day_3_data.txt").unwrap()
    );
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
    let mut counts = vec![0_usize; total_data_cols];
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

    let mut gamma_value = 0_u64;
    let mut epsilon_value = 0_u64;
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

    let mut current_bit = 0_usize;
    let mut current_values: Vec<&Vec<&str>> = values.iter().collect();
    while current_values.len() != 1 && !current_values.is_empty() {
        current_values = filter_values(current_values, current_bit, true);
        current_bit += 1;
    }
    let oxygen = bits_to_u64(current_values[0]);

    current_bit = 0_usize;
    let mut current_values: Vec<&Vec<&str>> = values.iter().collect();
    while current_values.len() != 1 && !current_values.is_empty() {
        current_values = filter_values(current_values, current_bit, false);
        current_bit += 1;
    }
    let co2 = bits_to_u64(current_values[0]);

    Ok(oxygen * co2)
}

fn bits_to_u64(chars: &[&str]) -> u64 {
    let mut result = 0_u64;
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

#[cfg(test)]
mod tests {
    use crate::{day_3_part_1, day_3_part_2};

    #[test]
    fn day_3_part_1_example() {
        let result = day_3_part_1("day_3_example.txt").unwrap();
        assert_eq!(result, 198);
    }

    #[test]
    fn day_3_part_2_example() {
        let result = day_3_part_2("day_3_example.txt").unwrap();
        assert_eq!(result, 230);
    }
}
