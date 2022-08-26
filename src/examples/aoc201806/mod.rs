use std::collections::{HashMap, HashSet};

use crate::math::manhattan_dist;

use super::super::grid::Grid;
use super::super::io::lines_from_file;

fn find_closest(x: i64, y: i64, grid: &mut Grid<i64>) -> Option<i64> {
    let mut min_dist = i64::MAX;
    let mut min_dist_count = 0;
    let mut min_pos = None;
    grid.walk(|pos, _v| {
        let dist = manhattan_dist(pos.0, pos.1, x, y);
        if min_dist == dist {
            min_dist_count += 1;
        } else if min_dist > dist {
            min_dist = dist;
            min_pos = Some(pos);
            min_dist_count = 0;
        }
    });

    if min_dist_count > 0 {
        return None;
    }

    if min_pos.is_none() {
        panic!("position not found");
    }

    return grid.get(min_pos.unwrap().0, min_pos.unwrap().1);
}

fn parse_grid(lines: &Vec<String>) -> Grid<i64> {
    let mut grid = Grid::empty();

    for (i, line) in lines.iter().enumerate() {
        let coords: Vec<i64> = line
            .split(", ")
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        println!("{}", i);
        grid.set(i as i64, coords[0], coords[1])
    }

    grid
}

#[allow(dead_code)]
fn solve1(filename: &str) -> i64 {
    let mut grid = parse_grid(&lines_from_file(filename));

    let mut excluded_areas = HashSet::new();
    let mut areas: HashMap<i64, i64> = HashMap::new();

    for x in 0..grid.max_x() + 1 {
        for y in 0..grid.max_y() + 1 {
            if let Some(closest) = find_closest(x, y, &mut grid) {
                areas.insert(closest, *areas.get(&closest).unwrap_or(&0) + 1);
                if x == 0 || x == grid.max_x() || y == 0 || y == grid.max_y() {
                    excluded_areas.insert(closest);
                }
            }
        }
    }

    let mut max_size = 0;
    for (area, count) in areas.iter() {
        if !excluded_areas.contains(area) {
            if max_size < *count {
                max_size = *count;
            }
        }
    }

    max_size
}

#[allow(dead_code)]
fn solve2(filename: &str, size: i64) -> i64 {
    let mut grid = parse_grid(&lines_from_file(filename));

    let mut area_size = 0;

    for x in 0..grid.max_x() + 1 {
        for y in 0..grid.max_y() + 1 {
            let mut area_sum = 0;
            grid.walk(|pos, _v| area_sum += manhattan_dist(pos.0, pos.1, x, y));
            if area_sum < size {
                area_size += 1;
            }
        }
    }

    area_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_closest() {
        let mut grid = Grid::<i64>::empty();
        grid.set(0, 4, 4);
        grid.set(1, 1, 1);
        grid.set(2, 3, 1);

        assert_eq!(Some(1), find_closest(0, 0, &mut grid));
        assert_eq!(Some(0), find_closest(5, 5, &mut grid));
        assert_eq!(None, find_closest(3, 3, &mut grid));
        assert_eq!(None, find_closest(2, 1, &mut grid));
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve1("src/examples/aoc201806/example.txt"), 17);
        assert_eq!(solve1("src/examples/aoc201806/input.txt"), 3909);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve2("src/examples/aoc201806/example.txt", 32), 16);
        assert_eq!(solve2("src/examples/aoc201806/input.txt", 10000), 36238);
    }
}
