use std::cmp::min;

use super::super::grid::Grid;
use super::super::io::lines_from_file;

fn coords(x: i64, y: i64, direction: &str, length: i64) -> Vec<(i64, i64)> {
    let mut r = vec![];
    match direction {
        "R" => {
            for nx in x + 1..x + length + 1 {
                r.push((nx, y));
            }
        }
        "L" => {
            for nx in x - length..x {
                r.push((nx, y));
            }
            r.reverse();
        }
        "D" => {
            for ny in y - length..y {
                r.push((x, ny));
            }
            r.reverse();
        }
        "U" => {
            for ny in y + 1..y + length + 1 {
                r.push((x, ny));
            }
        }
        _ => panic!("should not happen"),
    }

    r
}

fn distance(coord: (i64, i64)) -> u64 {
    (coord.0.abs() + coord.1.abs()) as u64
}

fn solve(filename: &str) -> (u64, u64) {
    let contents = lines_from_file(filename);
    let mut grid = Grid::<(u64, u64)>::empty();
    let mut min_distance = u64::MAX;
    let mut min_steps = u64::MAX;

    for (i, line) in contents.iter().enumerate() {
        let mut x = 0;
        let mut y = 0;
        let mut cur_steps = 0;

        for mov in line.split(',') {
            let direction: &str = &mov[0..1];
            let length = &mov[1..].parse::<i64>().unwrap();
            let coords = coords(x, y, direction, *length);

            for coord in coords {
                if coord == (0, 0) {
                    continue;
                }

                cur_steps += 1;

                if let Some(value) = grid.get(coord.0, coord.1) {
                    if value.0 != i as u64 && value.0 < 2 {
                        grid.set((2, cur_steps + value.1), coord.0, coord.1);

                        let new_distance = distance(coord);
                        min_distance = min(min_distance, new_distance);
                        min_steps = min(min_steps, cur_steps + value.1);
                    }
                } else {
                    grid.set((i as u64, cur_steps), coord.0, coord.1);
                }

                x = coord.0;
                y = coord.1;
            }
        }
    }

    (min_distance, min_steps)
}

#[allow(dead_code)]
fn solve1(filename: &str) -> u64 {
    solve(filename).0
}

#[allow(dead_code)]
fn solve2(filename: &str) -> u64 {
    solve(filename).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve1("src/examples/aoc201903/smallest.txt"), 6);
        assert_eq!(solve1("src/examples/aoc201903/small.txt"), 159);
        assert_eq!(solve1("src/examples/aoc201903/small2.txt"), 135);
        assert_eq!(solve1("src/examples/aoc201903/input.txt"), 446);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve2("src/examples/aoc201903/smallest.txt"), 30);
        assert_eq!(solve2("src/examples/aoc201903/small.txt"), 610);
        assert_eq!(solve2("src/examples/aoc201903/small2.txt"), 410);
        assert_eq!(solve2("src/examples/aoc201903/input.txt"), 9006);
    }
}
