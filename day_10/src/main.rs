use std::io::BufRead;

fn main() {
    let start = std::time::Instant::now();
    println!(
        "The total syntax error score for the errors is: {} in {}s",
        total_syntax_error_score("data.txt").unwrap(),
        start.elapsed().as_secs_f64()
    );
}

/// Check if line is corrupt returns the char position that failed if so.
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

#[cfg(test)]
mod tests {
    use crate::total_syntax_error_score;

    #[test]
    fn total_syntax_error_score_example() {
        let result = total_syntax_error_score("example.txt").unwrap();
        assert_eq!(result, 26397);
    }
}
