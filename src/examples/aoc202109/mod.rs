use std::collections::HashSet;

use super::super::graph::bfs;
use super::super::grid::{Grid, OFFSETS};
use super::super::io::lines_from_file;

fn find_lowest(grid: &Grid<char>) -> Vec<(i64, i64)> {
    let mut res = vec![];

    for y in 0..grid.max_y() {
        for x in 0..grid.max_x() {
            let mut is_lowest = true;
            let value = grid.get(x, y).unwrap_or('9');

            for o in OFFSETS.iter() {
                if let Some(neighbour) = grid.get(x + o.0, y + o.1) {
                    if neighbour <= value {
                        is_lowest = false;
                        break;
                    }
                }
            }

            if is_lowest {
                res.push((x, y));
            }
        }
    }

    res
}

#[allow(dead_code)]
fn solve1(filename: &str) -> u64 {
    let grid: Grid<char> = Grid::<char>::new(&lines_from_file(filename));

    let mut risk = 0;
    for pos in find_lowest(&grid) {
        risk += grid.get(pos.0, pos.1).unwrap_or('0').to_digit(10).unwrap() as u64 + 1;
    }

    risk
}

#[allow(dead_code)]
fn solve2(filename: &str) -> i64 {
    let grid: Grid<char> = Grid::<char>::new(&lines_from_file(filename));
    let mut values = HashSet::new();
    for value in ['0', '1', '2', '3', '4', '5', '6', '7', '8'] {
        values.insert(value);
    }

    let graph = grid.to_graph(values);
    let mut sizes = vec![];
    for pos in find_lowest(&grid) {
        sizes.push(bfs(&pos, &graph).len() as i64);
    }

    sizes.sort_by(|n1, n2| n2.cmp(n1));

    sizes[0] * sizes[1] * sizes[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve1("src/examples/aoc202109/example.txt"), 15);
        assert_eq!(solve1("src/examples/aoc202109/input.txt"), 475);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve2("src/examples/aoc202109/example.txt"), 1134);
        assert_eq!(solve2("src/examples/aoc202109/input.txt"), 1092012);
    }
}
