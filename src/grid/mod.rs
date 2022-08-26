pub mod hex;

use std::{
    cmp::max,
    cmp::min,
    collections::{HashMap, HashSet},
};

pub static RIGHT: (i64, i64) = (1, 0);
pub static LEFT: (i64, i64) = (-1, 0);
pub static BOTTOM: (i64, i64) = (0, 1);
pub static TOP: (i64, i64) = (0, -1);
pub static TOP_LEFT: (i64, i64) = (-1, -1);
pub static BOTTOM_RIGHT: (i64, i64) = (1, 1);
pub static TOP_RIGHT: (i64, i64) = (1, -1);
pub static BOTTOM_LEFT: (i64, i64) = (-1, 1);

lazy_static! {
    pub static ref OFFSETS: Vec<(i64, i64)> = vec![
        RIGHT,
        LEFT,
        BOTTOM,
        TOP,
        TOP_LEFT,
        BOTTOM_RIGHT,
        TOP_RIGHT,
        BOTTOM_LEFT,
    ];
    pub static ref OFFSETS_STRAIGHT: Vec<(i64, i64)> = vec![RIGHT, LEFT, TOP, BOTTOM];
}

#[derive(PartialEq, Eq)]
pub struct Grid<T> {
    data: HashMap<(i64, i64), T>,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl<T> Grid<T> {
    pub fn empty() -> Grid<T> {
        Grid {
            data: HashMap::new(),
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
        }
    }

    pub fn new(input: &Vec<String>) -> Grid<char> {
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

    pub fn get(&self, x: i64, y: i64) -> Option<T>
    where
        T: Copy,
    {
        match self.data.get(&(x, y)) {
            Some(c) => Some(*c),
            None => None,
        }
    }

    pub fn set(&mut self, input: T, x: i64, y: i64) {
        self.data.insert((x, y), input);
        self.min_x = min(self.min_x, x);
        self.max_x = max(self.max_x, x);
        self.min_y = min(self.min_y, y);
        self.max_y = max(self.max_y, y);
    }

    fn prepare_print(&self) -> Vec<String>
    where
        T: std::fmt::Display + Copy,
    {
        let mut output = vec![];
        for y in self.min_y..self.max_y {
            let mut line = String::new();
            for x in self.min_x..self.max_x {
                match self.get(x, y) {
                    Some(c) => line.push_str(&format!("{}", c)),
                    _ => line.push('.'),
                }
            }
            output.push(line);
        }

        output
    }

    pub fn print(&self)
    where
        T: std::fmt::Display + Copy,
    {
        for line in self.prepare_print() {
            println!("{}", line);
        }
    }

    pub fn walk<F>(&mut self, mut visitor: F)
    where
        T: Copy,
        F: FnMut((i64, i64), T),
    {
        for (k, v) in &self.data {
            visitor(*k, *v)
        }
    }

    pub fn min_x(&self) -> i64 {
        self.min_x
    }

    pub fn max_x(&self) -> i64 {
        self.max_x
    }

    pub fn min_y(&self) -> i64 {
        self.min_y
    }

    pub fn max_y(&self) -> i64 {
        self.max_y
    }

    pub fn to_graph(&self, values: HashSet<T>) -> HashMap<(i64, i64), Vec<(i64, i64)>>
    where
        T: Copy + Eq + std::hash::Hash,
    {
        let mut graph = HashMap::new();

        for (pos, value) in self.data.iter() {
            if values.contains(value) {
                if !graph.contains_key(pos) {
                    let n = vec![];
                    graph.insert(*pos, n);
                }

                let neighbours = graph.get_mut(pos).unwrap();

                for dir in OFFSETS_STRAIGHT.iter() {
                    let neighbour_pos = (dir.0 + pos.0, dir.1 + pos.1);
                    if let Some(neighbour_value) = self.get(neighbour_pos.0, neighbour_pos.1) {
                        if values.contains(&neighbour_value) {
                            neighbours.push(neighbour_pos);
                        }
                    }
                }
            }
        }

        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input<'a>() -> Vec<String> {
        let mut data: Vec<String> = vec![];
        data.push(".......#................#......".to_string());
        data.push("...#.#.....#.##.....#..#.......".to_string());
        data.push("..#..#.#......#.#.#............".to_string());
        data.push("....#...#...##.....#..#.....#..".to_string());
        data.push("....#.......#.##......#...#..#.".to_string());
        data.push("...............#.#.#.....#..#..".to_string());
        data.push("...##...#...#..##.###...##.....".to_string());
        data.push("##..#.#...##.....#.#..........#".to_string());
        data.push(".#....#..#..#......#....#....#.".to_string());

        data
    }

    #[test]
    fn test_empty() {
        let grid: Grid<char> = Grid::empty();
        assert_eq!(0, grid.min_x);
        assert_eq!(0, grid.max_x);
        assert_eq!(0, grid.min_y);
        assert_eq!(0, grid.max_y);
    }

    #[test]
    fn test_new() {
        let grid: Grid<char> = Grid::<char>::new(&get_input());

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
        let grid: Grid<char> = Grid::<char>::new(&get_input());

        assert_eq!(Some('.'), grid.get(0, 0));
        assert_eq!(Some('#'), grid.get(3, 1));
        assert_eq!(None, grid.get(-3000, -3000));
    }

    #[test]
    fn test_print() {
        let input = get_input();
        let grid = Grid::<char>::new(&input);

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

    #[test]
    fn test_to_graph() {
        let grid: Grid<char> = Grid::<char>::new(&get_input());
        let mut values = HashSet::new();
        values.insert('.');

        let graph = grid.to_graph(values);

        assert_eq!(218, graph.len());
        let neighours = graph.get(&(0, 0)).unwrap();
        assert_eq!(2, neighours.len());
        assert_eq!((1, 0), neighours[0]);
        assert_eq!((0, 1), neighours[1]);
    }
}
