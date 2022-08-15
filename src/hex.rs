use std::collections::HashMap;

//   \ n  /
// nw +--+ ne
//   /    \
// -+      +-
//   \    /
// sw +--+ se
//   / s  \

// https://www.redblobgames.com/grids/hexagons/#coordinates-cube
lazy_static! {
    pub static ref HEX_OFFSETS: HashMap<&'static str, (i64, i64, i64)> = vec![
        ("n", (1, -1, 0)),
        ("s", (-1, 1, 0)),
        ("nw", (0, -1, 1)),
        ("ne", (1, 0, -1)),
        ("se", (0, 1, -1)),
        ("sw", (-1, 0, 1)),
    ]
    .into_iter()
    .collect();
}

pub fn hex_cube_add(a: (i64, i64, i64), b: (i64, i64, i64)) -> (i64, i64, i64) {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

pub fn hex_cube_subtract(a: (i64, i64, i64), b: (i64, i64, i64)) -> (i64, i64, i64) {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

// https://www.redblobgames.com/grids/hexagons/#distances
pub fn hex_cube_dist(a: (i64, i64, i64), b: (i64, i64, i64)) -> i64 {
    let diff = hex_cube_subtract(a, b);
    (diff.0.abs() + diff.1.abs() + diff.2.abs()) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = hex_cube_add((0, 0, 0), *HEX_OFFSETS.get(&"n").unwrap());
        assert_eq!((1, -1, 0), result)
    }

    #[test]
    fn test_subtract() {
        let result = hex_cube_subtract((0, 0, 0), *HEX_OFFSETS.get(&"n").unwrap());
        assert_eq!(*HEX_OFFSETS.get(&"s").unwrap(), result)
    }

    #[test]
    fn test_dist() {
        let result = hex_cube_dist(
            *HEX_OFFSETS.get(&"sw").unwrap(),
            *HEX_OFFSETS.get(&"ne").unwrap(),
        );
        assert_eq!(2, result);
    }
}
