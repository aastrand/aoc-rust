pub mod hex;

use std::{cmp::max, cmp::min, collections::HashMap};

pub static RIGHT: (i64, i64) = (1, 0);
pub static LEFT: (i64, i64) = (-1, 0);
pub static BOTTOM: (i64, i64) = (0, 1);
pub static TOP: (i64, i64) = (0, -1);
pub static TOP_LEFT: (i64, i64) = (-1, -1);
pub static BOTTOM_RIGHT: (i64, i64) = (1, 1);
pub static TOP_RIGHT: (i64, i64) = (1, -1);
pub static BOTTOM_LEFT: (i64, i64) = (-1, 1);

pub static OFFSETS: (
    (i64, i64),
    (i64, i64),
    (i64, i64),
    (i64, i64),
    (i64, i64),
    (i64, i64),
    (i64, i64),
    (i64, i64),
) = (
    RIGHT,
    LEFT,
    BOTTOM,
    TOP,
    TOP_LEFT,
    BOTTOM_RIGHT,
    TOP_RIGHT,
    BOTTOM_LEFT,
);
pub static OFFSETS_STRAIGHT: ((i64, i64), (i64, i64), (i64, i64), (i64, i64)) =
    (RIGHT, LEFT, TOP, BOTTOM);

pub struct Grid {
    data: HashMap<(i64, i64), char>,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl Grid {
    pub fn empty() -> Grid {
        Grid::new(&vec![])
    }

    pub fn new(input: &Vec<&str>) -> Grid {
        let height = input.len();
        let width = if height > 0 { input[0].len() } else { 0 };
        let mut data = HashMap::new();

        for (y, line) in input.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                data.insert((x as i64, y as i64), char);
            }
        }

        Grid {
            data,
            min_x: 0,
            max_x: width as i64,
            min_y: 0,
            max_y: height as i64,
        }
    }

    pub fn get(&self, x: i64, y: i64) -> Option<char> {
        match self.data.get(&(x, y)) {
            Some(c) => Some(*c),
            None => None,
        }
    }

    pub fn set(&mut self, input: char, x: i64, y: i64) {
        self.data.insert((x, y), input);
        self.min_x = min(self.min_x, x);
        self.max_x = max(self.max_x, x);
        self.min_y = min(self.min_y, y);
        self.max_y = max(self.max_y, y);
    }

    fn prepare_print(&self) -> Vec<String> {
        let mut output = vec![];
        for y in self.min_y..self.max_y {
            let mut line = String::new();
            for x in self.min_x..self.max_x {
                match self.get(x, y) {
                    Some(c) => line.push(c),
                    _ => line.push('.'),
                }
            }
            output.push(line);
        }

        output
    }

    pub fn print(&self) {
        for line in self.prepare_print() {
            println!("{}", line);
        }
    }

    pub fn walk<F>(&mut self, mut visitor: F)
    where
        F: FnMut((i64, i64), char),
    {
        for (k, v) in &self.data {
            visitor(*k, *v)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input<'a>() -> Vec<&'a str> {
        let mut data = vec![];
        data.push(".......#................#......");
        data.push("...#.#.....#.##.....#..#.......");
        data.push("..#..#.#......#.#.#............");
        data.push("....#...#...##.....#..#.....#..");
        data.push("....#.......#.##......#...#..#.");
        data.push("...............#.#.#.....#..#..");
        data.push("...##...#...#..##.###...##.....");
        data.push("##..#.#...##.....#.#..........#");
        data.push(".#....#..#..#......#....#....#.");

        data
    }

    #[test]
    fn test_empty() {
        let grid = Grid::empty();
        assert_eq!(0, grid.min_x);
        assert_eq!(0, grid.max_x);
        assert_eq!(0, grid.min_y);
        assert_eq!(0, grid.max_y);
    }

    #[test]
    fn test_new() {
        let grid = Grid::new(&get_input());

        assert_eq!(0, grid.min_x);
        assert_eq!(31, grid.max_x);
        assert_eq!(0, grid.min_y);
        assert_eq!(9, grid.max_y);
    }

    #[test]
    fn test_set() {
        let mut grid = Grid::empty();
        grid.set('#', -10, 23);
        grid.set('#', 100, -2);
        assert_eq!(-10, grid.min_x);
        assert_eq!(100, grid.max_x);
        assert_eq!(-2, grid.min_y);
        assert_eq!(23, grid.max_y);
    }

    #[test]
    fn test_get() {
        let grid = Grid::new(&get_input());

        assert_eq!(Some('.'), grid.get(0, 0));
        assert_eq!(Some('#'), grid.get(3, 1));
        assert_eq!(None, grid.get(-3000, -3000));
    }

    #[test]
    fn test_print() {
        let input = get_input();
        let grid = Grid::new(&input);

        let output = grid.prepare_print();

        assert_eq!(input.len(), output.len());
        for i in 0..output.len() {
            assert_eq!(input[i], output[i].as_str());
        }

        grid.print();
    }

    #[test]
    fn test_count() {
        let mut grid = Grid::empty();
        grid.set('#', -10, 23);
        grid.set('#', 100, -2);
        grid.set('.', -10, 23);
        grid.set('.', 0, 0);

        let mut count = 0;
        grid.walk(|_pos, val| {
            if val == '#' {
                count += 1;
            }
        });

        assert_eq!(1, count);
    }
}
