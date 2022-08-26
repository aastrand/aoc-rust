pub fn manhattan_dist(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    (x2 - x1).abs() + (y2 - y1).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_dist() {
        assert_eq!(0, manhattan_dist(0, 1, 0, 1));
        assert_eq!(20, manhattan_dist(0, 0, 10, 10));
        assert_eq!(20, manhattan_dist(-5, -5, 5, 5));
    }
}
