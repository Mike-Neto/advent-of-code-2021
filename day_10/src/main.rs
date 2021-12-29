use std::io::BufRead;

fn main() {
    let mut start = std::time::Instant::now();
    println!(
        "The total syntax error score for the errors is: {} in {}s",
        total_syntax_error_score("data.txt").unwrap(),
        start.elapsed().as_secs_f64()
    );

    start = std::time::Instant::now();
    println!(
        "The middle score for the incomplete lines is: {} in {}s",
        middle_incomplete_score("data.txt").unwrap(),
        start.elapsed().as_secs_f64()
    );
}

/// Check if line is corrupt returns the score.
fn is_corrupt(line: &str) -> Option<usize> {
    let mut queue: Vec<char> = vec![];
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => queue.push(c),
            ')' | ']' | '}' | '>' => {
                if let Some(tail) = queue.pop() {
                    let is_close_parentheses = tail == '(' && c == ')';
                    let is_close_rect = tail == '[' && c == ']';
                    let is_close_curly = tail == '{' && c == '}';
                    let is_close_angle = tail == '<' && c == '>';
                    if !is_close_parentheses && !is_close_rect && !is_close_curly && !is_close_angle
                    {
                        let value = if c == ')' {
                            3
                        } else if c == ']' {
                            57
                        } else if c == '}' {
                            1197
                        } else {
                            25137
                        };
                        return Some(value);
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    None
}

fn total_syntax_error_score(path: &str) -> Result<usize, std::io::Error> {
    let score = std::fs::read(path)?
        .lines()
        .filter_map(std::result::Result::ok)
        .filter_map(|line| is_corrupt(&line))
        .sum();

    Ok(score)
}

/// Check if line is incomplete and returns the score.
fn is_incomplete(line: &str) -> Option<usize> {
    let mut queue: Vec<char> = vec![];
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => queue.push(c),
            ')' | ']' | '}' | '>' => {
                if let Some(tail) = queue.pop() {
                    let is_close_parentheses = tail == '(' && c == ')';
                    let is_close_rect = tail == '[' && c == ']';
                    let is_close_curly = tail == '{' && c == '}';
                    let is_close_angle = tail == '<' && c == '>';
                    if !is_close_parentheses && !is_close_rect && !is_close_curly && !is_close_angle
                    {
                        return None;
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    // Check what is missing
    let score = queue
        .iter()
        .map(|&open| match open {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => unreachable!(),
        })
        .rev()
        .fold(0_usize, |mut total_score, close| {
            total_score *= 5;
            let value = match close {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => unreachable!(),
            };
            total_score += value;
            total_score
        });
    Some(score)
}

fn middle_incomplete_score(path: &str) -> Result<usize, std::io::Error> {
    let mut score = std::fs::read(path)?
        .lines()
        .filter_map(std::result::Result::ok)
        .filter_map(|line| is_incomplete(&line))
        .collect::<Vec<usize>>();
    score.sort_unstable();
    Ok(score[score.len() / 2])
}

#[cfg(test)]
mod tests {
    use crate::{middle_incomplete_score, total_syntax_error_score};

    #[test]
    fn total_syntax_error_score_example() {
        let result = total_syntax_error_score("example.txt").unwrap();
        assert_eq!(result, 26397);
    }

    #[test]
    fn middle_incomplete_score_example() {
        let result = middle_incomplete_score("example.txt").unwrap();
        assert_eq!(result, 288_957);
    }
}
