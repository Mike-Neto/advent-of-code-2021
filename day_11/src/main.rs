use std::io::BufRead;
const NEXT: [(isize, isize); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];
const SIZE: usize = 10;

fn main() {
    let mut start = std::time::Instant::now();
    let steps = 100;
    println!(
        "The total amount of flashes after {} steps is: {} in {}s",
        steps,
        total_flashes("data.txt", steps).unwrap(),
        start.elapsed().as_secs_f64()
    );

    start = std::time::Instant::now();
    println!(
        "The total amount of steps until a full flash is: {} in {}s",
        steps_until_full_flash("data.txt").unwrap(),
        start.elapsed().as_secs_f64()
    );
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
fn propagate(map: &mut Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    map[y][x] = 0;
    NEXT.iter()
        // Note these casts here are safe as I don't mind if it wraps or not since I will use it to index into the grid
        // and even if the value wraps its just a out of bounds just like it would be if it was -1
        .map(|(xx, yy)| ((x as isize + xx) as usize, (y as isize + yy) as usize))
        .fold(1, |acc, (x, y)| {
            match map.get_mut(y).and_then(|l| l.get_mut(x)) {
                Some(cell) if *cell > 0 => {
                    *cell += 1;
                    acc + (*cell > 9).then(|| propagate(map, x, y)).unwrap_or(0)
                }
                _ => acc,
            }
        })
}

fn total_flashes(path: &str, steps: usize) -> Result<usize, std::io::Error> {
    let mut m: Vec<Vec<u8>> = std::fs::read(path)?
        .lines()
        .filter_map(std::result::Result::ok)
        .map(|line| line.split("").filter_map(|c| c.parse().ok()).collect())
        .collect();

    let sum = (0..steps).fold(0, |acc, _step| {
        m.iter_mut()
            .for_each(|row| row.iter_mut().for_each(|cell| *cell += 1));
        acc + (0..SIZE)
            .flat_map(|y| (0..SIZE).map(move |x| (x, y)))
            .fold(0, |acc, (x, y)| {
                acc + (m[y][x] > 9).then(|| propagate(&mut m, x, y)).unwrap_or(0)
            })
    });

    Ok(sum)
}

fn steps_until_full_flash(path: &str) -> Result<usize, std::io::Error> {
    let mut m: Vec<Vec<u8>> = std::fs::read(path)?
        .lines()
        .filter_map(std::result::Result::ok)
        .map(|line| line.split("").filter_map(|c| c.parse().ok()).collect())
        .collect();

    let sum = (1..usize::MAX)
        .find(|_step| {
            m.iter_mut()
                .for_each(|row| row.iter_mut().for_each(|cell| *cell += 1));
            (0..SIZE)
                .flat_map(|y| (0..SIZE).map(move |x| (x, y)))
                .fold(0, |acc, (x, y)| {
                    acc + (m[y][x] > 9).then(|| propagate(&mut m, x, y)).unwrap_or(0)
                })
                == SIZE * SIZE
        })
        .unwrap();

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use crate::{steps_until_full_flash, total_flashes};

    #[test]
    fn total_flashes_example() {
        let mut result = total_flashes("example.txt", 10).unwrap();
        assert_eq!(result, 204);
        result = total_flashes("example.txt", 100).unwrap();
        assert_eq!(result, 1656);
    }

    #[test]
    fn steps_until_full_flash_example() {
        let result = steps_until_full_flash("example.txt").unwrap();
        assert_eq!(result, 195);
    }
}
