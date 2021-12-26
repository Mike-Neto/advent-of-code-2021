fn main() {
    println!(
        "The digits 1, 4, 7, or 8 appear {} times",
        count_digits("data.txt").unwrap()
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

#[cfg(test)]
mod tests {
    use crate::count_digits;

    #[test]
    fn count_digits_example() {
        let result = count_digits("example.txt").unwrap();
        assert_eq!(result, 26);
    }
}
