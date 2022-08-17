#[allow(unused_imports)]
use super::super::io::lines_from_file;
use super::super::parse::ints;

#[allow(dead_code)]
fn solve(filename: &str, part2: bool) -> i64 {
    let lines = lines_from_file(filename);
    let mut discs = vec![];
    for line in lines {
        let d = ints(&line);
        discs.push((d[1], d[3]))
    }

    if part2 {
        discs.push((11, 0))
    }

    let mut t = 0;
    loop {
        let mut valid = true;
        for (i, d) in discs.iter().enumerate() {
            if (d.1 + i as i64 + t + 1) % d.0 != 0 {
                valid = false;
                break;
            }
        }

        if valid {
            return t;
        }

        t += 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(5, solve("src/examples/aoc201615example.txt", false));
        assert_eq!(400589, solve("src/examples/aoc201615input.txt", false));
    }

    #[test]
    fn test_part2() {
        assert_eq!(3045959, solve("src/examples/aoc201615input.txt", true));
    }
}
