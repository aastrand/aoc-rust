use std::collections::{HashMap, HashSet, VecDeque};

use crate::graph::dijkstra::dijkstra;
use crate::grid::OFFSETS_STRAIGHT;

#[allow(unused_imports)]
use super::super::graph::dijkstra;
use super::super::grid::Grid;

fn get_grid(input: i64, max_x: i64, max_y: i64) -> Grid {
    let mut grid = Grid::empty();
    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            let mut val: i64 = (x * x) + (3 * x) + (2 * x * y) + y + (y * y);
            val += input;
            let bits: i64 = format!("{:b}", val)
                .chars()
                .map(|b| match b {
                    '0' => 0,
                    _ => 1,
                })
                .sum();

            grid.set(
                match bits % 2 {
                    0 => '.',
                    _ => '#',
                },
                x,
                y,
            );
        }
    }

    grid
}

fn build_graph<'a>(grid: Grid) -> HashMap<(i64, i64), Vec<(i64, i64)>> {
    let mut graph: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();
    for y in 0..grid.max_y() {
        for x in 0..grid.max_x() {
            for o in OFFSETS_STRAIGHT.iter() {
                let n = (x + o.0, y + o.1);
                if let Some(val) = grid.get(n.0, n.1) {
                    if val == '.' {
                        if let Some(neighours) = graph.get_mut(&(x, y)) {
                            neighours.push(n);
                        } else {
                            let mut neighbours = vec![];
                            neighbours.push(n);
                            graph.insert((x, y), neighbours);
                        }
                    }
                }
            }
        }
    }

    graph
}

#[allow(dead_code)]
fn part1(input: i64, target: (i64, i64)) -> i64 {
    let grid = get_grid(input, target.0 + 10, target.1 + 10);
    let graph = build_graph(grid);

    let (path, _dist, _prev) = dijkstra(&(1, 1), &target, &graph);

    path.len() as i64 - 1
}

fn get_path<'a>(
    n: &'a (i64, i64),
    prev: &HashMap<&'a (i64, i64), &'a (i64, i64)>,
) -> Vec<&'a (i64, i64)> {
    let mut path = vec![];
    let mut u = n;
    if prev.contains_key(u) {
        loop {
            path.push(u);
            if let Some(next) = prev.get(u) {
                u = next;
            } else {
                break;
            }
        }
    }

    path.reverse();

    path
}

fn bfs<'a>(
    start: &'a (i64, i64),
    graph: &'a HashMap<(i64, i64), Vec<(i64, i64)>>,
    max: usize,
) -> HashSet<&'a (i64, i64)> {
    let mut visited = HashSet::new();
    let mut q = VecDeque::new();

    q.push_back(start);
    visited.insert(start);

    let (_graph, _dist, prev) = dijkstra(start, start, graph);

    while q.len() > 0 {
        let cur = q.pop_front().unwrap();
        if let Some(neighbours) = graph.get(cur) {
            for n in neighbours {
                if !visited.contains(n) {
                    let path = get_path(&n, &prev);
                    if path.len() - 1 <= max {
                        q.push_back(n);
                        visited.insert(n);
                    }
                }
            }
        }
    }

    visited
}

#[allow(dead_code)]
fn part2(input: i64, size: i64) -> i64 {
    let grid = get_grid(input, size, size);
    let graph = build_graph(grid);

    let visited = bfs(&(1, 1), &graph, 50);

    visited.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_grid() {
        let grid = get_grid(10, 10, 10);

        assert_eq!('.', grid.get(0, 0).unwrap());
        assert_eq!('#', grid.get(1, 0).unwrap());
        assert_eq!('.', grid.get(2, 0).unwrap());
        assert_eq!('#', grid.get(3, 0).unwrap());

        assert_eq!('#', grid.get(4, 3).unwrap());
        assert_eq!('.', grid.get(5, 3).unwrap());
        assert_eq!('#', grid.get(6, 3).unwrap());
        assert_eq!('#', grid.get(7, 3).unwrap());

        assert_eq!('.', grid.get(6, 6).unwrap());
        assert_eq!('#', grid.get(7, 6).unwrap());
        assert_eq!('#', grid.get(8, 6).unwrap());
        assert_eq!('#', grid.get(9, 6).unwrap());
    }

    #[test]
    fn test_part1() {
        assert_eq!(11, part1(10, (7, 4)));
        assert_eq!(86, part1(1364, (31, 39)));
    }

    #[test]
    fn test_part2() {
        // too costly in debug
        // assert_eq!(127, part2(1364, 100));
        assert_eq!(40, part2(1364, 10));
    }
}
