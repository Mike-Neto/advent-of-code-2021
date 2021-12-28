fn main() {
    println!(
        "The digits 1, 4, 7, or 8 appear {} times",
        count_digits("data.txt").unwrap()
    );

    println!(
        "The sum for all output values is: {}",
        sum_output_values("data.txt").unwrap()
    );
}

fn count_digits(path: &str) -> Result<usize, std::io::Error> {
    let file = std::fs::read_to_string(path)?;
    let output_digits = file
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|line| {
            line.split('|')
                .map(|piece| piece.trim())
                .collect::<Vec<&str>>()
        })
        .map(|line| line[1])
        .map(|output_segments| {
            output_segments
                .split_whitespace()
                .filter(|&segment| !segment.is_empty())
                .collect::<Vec<&str>>()
        })
        .flatten()
        .filter(|segment| {
            segment.len() == 7 || segment.len() == 4 || segment.len() == 3 || segment.len() == 2
        });

    Ok(output_digits.count())
}

fn sum_output_values(path: &str) -> Result<usize, std::io::Error> {
    let file = std::fs::read_to_string(path)?;
    let sum =
        file.split('\n')
            .filter(|s| !s.is_empty())
            .map(|line| {
                line.split('|')
                    .map(|piece| piece.trim())
                    .collect::<Vec<&str>>()
            })
            .map(|line| {
                line.into_iter()
                    .map(|segment| {
                        segment
                            .split_whitespace()
                            .filter(|&segment| !segment.is_empty())
                            .collect::<Vec<&str>>()
                    })
                    .collect::<Vec<Vec<&str>>>()
            })
            .fold(0, |mut sum, line| {
                let signals = &line[0];
                let segments = &line[1];

                let &one = signals.iter().find(|signal| signal.len() == 2).unwrap();
                let &four = signals.iter().find(|signal| signal.len() == 4).unwrap();
                let &seven = signals.iter().find(|signal| signal.len() == 3).unwrap();
                let &eight = signals.iter().find(|signal| signal.len() == 7).unwrap();
                let one_chars = one.chars().collect::<Vec<char>>();
                let eight_chars = eight.chars().collect::<Vec<char>>();
                let four_chars = four.chars().collect::<Vec<char>>();

                let segment_values: Vec<u8> = segments
                    .iter()
                    .map(|&segment| {
                        let segment_len = segment.len();
                        let is_one = one.len() == segment_len;
                        if is_one {
                            return 1;
                        }
                        let is_four = four.len() == segment_len;
                        if is_four {
                            return 4;
                        }
                        let is_seven = seven.len() == segment_len;
                        if is_seven {
                            return 7;
                        }
                        let is_eight = eight.len() == segment_len;
                        if is_eight {
                            return 8;
                        }

                        use array_tool::vec::Uniq;
                        let segment_chars = segment.chars().collect::<Vec<char>>();
                        if segment_len == 6 {
                            let eight_uniques = segment_chars.uniq(eight_chars.clone());
                            let four_uniques = segment_chars.uniq(four_chars.clone());
                            let one_uniques = segment_chars.uniq(one_chars.clone());
                            let is_nine = eight_uniques.is_empty() && four_uniques.len() == 2;
                            if is_nine {
                                return 9;
                            }
                            let is_six = eight_uniques.is_empty()
                                && four_uniques.len() == 3
                                && one_uniques.len() == 5;
                            if is_six {
                                return 6;
                            }
                        }

                        if segment_len == 5 {
                            let eight_uniques = segment_chars.uniq(eight_chars.clone());
                            let four_uniques = segment_chars.uniq(four_chars.clone());
                            let one_uniques = segment_chars.uniq(one_chars.clone());
                            let is_five = eight_uniques.is_empty()
                                && four_uniques.len() == 2
                                && one_uniques.len() == 4;
                            if is_five {
                                return 5;
                            }
                            let is_two = eight_uniques.is_empty() && four_uniques.len() == 3;
                            if is_two {
                                return 2;
                            }
                            let is_three = eight_uniques.is_empty()
                                && four_uniques.len() == 2
                                && one_uniques.len() == 3;
                            if is_three {
                                return 3;
                            }
                        }
                        0
                    })
                    .collect();

                println!("{:?}", segment_values);

                let value = segment_values.iter().rev().enumerate().fold(
                    0usize,
                    |mut acc, (index, &digit)| {
                        let magnitude = usize::pow(10, index as u32);
                        acc += digit as usize * magnitude;
                        acc
                    },
                );

                sum += value;
                sum
            });

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use crate::{count_digits, sum_output_values};

    #[test]
    fn count_digits_example() {
        let result = count_digits("example.txt").unwrap();
        assert_eq!(result, 26);
    }

    #[test]
    fn sum_output_values_example() {
        let result = sum_output_values("example.txt").unwrap();
        assert_eq!(result, 61229);
    }

    #[test]
    fn sum_output_values_attempt_1() {
        let result = sum_output_values("data.txt").unwrap();
        assert_ne!(result, 1169712);
        assert!(result < 1169712);
    }
}
