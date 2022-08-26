use super::super::grid::{Grid, OFFSETS};
use super::super::io::lines_from_file;

fn count_char(chr: char, grid: &mut Grid<char>) -> u64 {
    let mut sum = 0;

    grid.walk(|_k, v| {
        if v == chr {
            sum += 1;
        }
    });

    sum
}

fn num_adjecent(chr: char, y: i64, x: i64, grid: &Grid<char>) -> u64 {
    let mut sum = 0;

    for pos in OFFSETS.iter() {
        let new_x = pos.0 + x;
        let new_y = pos.1 + y;
        match grid.get(new_x, new_y) {
            Some(c) => {
                if c == chr {
                    sum += 1
                }
            }
            _ => (),
        }
    }

    sum
}

/*
If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
Otherwise, the seat's state does not change.
Floor (.) never changes; seats don't move, and nobody sits on the floor.
*/
fn mutate_grid(
    grid: &Grid<char>,
    empty_mutation: &dyn Fn(i64, i64, &Grid<char>) -> char,
    occupied_mutation: &dyn Fn(i64, i64, &Grid<char>) -> char,
) -> Grid<char> {
    let mut new_grid = Grid::empty();

    for y in 0..grid.max_y() + 1 {
        for x in 0..grid.max_x() + 1 {
            let pos = match grid.get(x, y) {
                Some('L') => Some(empty_mutation(y, x, &grid)),
                Some('#') => Some(occupied_mutation(y, x, &grid)),
                c => c,
            };
            if let Some(new) = pos {
                new_grid.set(new, x, y);
            }
        }
    }

    new_grid
}

fn empty_mutation_1(y: i64, x: i64, grid: &Grid<char>) -> char {
    if num_adjecent('#', y, x, &grid) == 0 {
        '#'
    } else {
        'L'
    }
}

fn occupied_mutation_1(y: i64, x: i64, grid: &Grid<char>) -> char {
    if num_adjecent('#', y, x, &grid) >= 4 {
        'L'
    } else {
        '#'
    }
}

#[allow(dead_code)]
fn solve1(filename: &str) -> u64 {
    let mut grid: Grid<char> = Grid::<char>::new(&lines_from_file(filename));

    loop {
        let new_grid = mutate_grid(&grid, &empty_mutation_1, &occupied_mutation_1);
        if new_grid == grid {
            break;
        }
        grid = new_grid;
    }

    count_char('#', &mut grid)
}

fn num_visible_adjecent(chr: char, y: i64, x: i64, grid: &Grid<char>) -> u64 {
    let mut sum = 0;

    for pos in OFFSETS.iter() {
        let mut new_x = pos.0 + x;
        let mut new_y = pos.1 + y;
        while new_y >= 0 && new_y < grid.max_y() + 1 && new_x >= 0 && new_x < grid.max_x() + 1 {
            if let Some(f) = grid.get(new_x, new_y) {
                if f != '.' {
                    if f == chr {
                        sum += 1;
                    }

                    break;
                }
            }
            new_x += pos.0;
            new_y += pos.1;
        }
    }

    sum
}

fn empty_mutation_2(y: i64, x: i64, grid: &Grid<char>) -> char {
    if num_visible_adjecent('#', y, x, &grid) == 0 {
        '#'
    } else {
        'L'
    }
}

fn occupied_mutation_2(y: i64, x: i64, grid: &Grid<char>) -> char {
    if num_visible_adjecent('#', y, x, &grid) >= 5 {
        'L'
    } else {
        '#'
    }
}

#[allow(dead_code)]
fn solve2(filename: &str) -> u64 {
    let mut grid: Grid<char> = Grid::<char>::new(&lines_from_file(filename));

    loop {
        let new_grid = mutate_grid(&grid, &empty_mutation_2, &occupied_mutation_2);
        if new_grid == grid {
            break;
        }
        grid = new_grid;
    }

    count_char('#', &mut grid)
}
#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve1("src/examples/aoc202011/example.txt"), 37);
        assert_eq!(solve1("src/examples/aoc202011/input.txt"), 2283);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve2("src/examples/aoc202011/example.txt"), 26);
        assert_eq!(solve2("src/examples/aoc202011/input.txt"), 2054);
    }
}
