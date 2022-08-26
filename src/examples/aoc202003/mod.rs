use super::super::grid::Grid;
use super::super::io::lines_from_file;

fn get_grid(filename: &str) -> Grid<char> {
    let contents = lines_from_file(filename);

    let height = contents.len();
    let input_width = contents[0].len();
    let extensions = ((height * 7) / input_width) + 1;
    let mut grid = Grid::empty();

    for (y, line) in contents.iter().enumerate() {
        for extension in 0..extensions {
            for (x, chr) in line.chars().enumerate() {
                grid.set(chr, x as i64 + (extension * input_width) as i64, y as i64);
            }
        }
    }

    grid
}

#[allow(dead_code)]
fn solve1(filename: &str) -> u64 {
    let grid = get_grid(filename);

    count_trees(&grid, 1, 3)
}

fn count_trees(grid: &Grid<char>, down: i64, right: i64) -> u64 {
    let mut x = 0;
    let mut y = 0;
    let mut result = 0;

    while y < grid.max_y() + 1 {
        match grid.get(x, y) {
            Some('#') => result += 1,
            _ => (),
        }

        x += right;
        y += down;
    }

    result
}

#[allow(dead_code)]
fn solve2(filename: &str) -> u64 {
    let grid = get_grid(filename);

    count_trees(&grid, 1, 1)
        * count_trees(&grid, 1, 3)
        * count_trees(&grid, 1, 5)
        * count_trees(&grid, 1, 7)
        * count_trees(&grid, 2, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve1("src/examples/aoc202003/example.txt"), 7);
        assert_eq!(solve1("src/examples/aoc202003/input.txt"), 207);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve2("src/examples/aoc202003/example.txt"), 336);
        assert_eq!(solve2("src/examples/aoc202003/input.txt"), 2655892800);
    }
}
