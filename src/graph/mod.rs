pub mod dijkstra;

use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

pub fn bfs<'a, T>(start: &'a T, graph: &'a HashMap<T, Vec<T>>) -> HashSet<&'a T>
where
    T: Eq + Hash,
{
    bfs_with_visitor(start, graph, |_n| {})
}

pub fn bfs_with_visitor<'a, T, F>(
    start: &'a T,
    graph: &'a HashMap<T, Vec<T>>,
    mut visitor: F,
) -> HashSet<&'a T>
where
    T: Eq + Hash,
    F: FnMut(&'a T),
{
    let mut visited: HashSet<&'a T> = HashSet::new();
    let mut q: VecDeque<&'a T> = VecDeque::new();

    q.push_back(start);
    visited.insert(start);
    visitor(start);

    while q.len() > 0 {
        let cur = q.pop_front().unwrap();
        if let Some(neighbours) = graph.get(cur) {
            for n in neighbours {
                if !visited.contains(n) {
                    q.push_back(n);
                    visited.insert(n);
                    visitor(n);
                }
            }
        }
    }

    visited
}

pub fn dfs<'a, T>(start: &'a T, graph: &'a HashMap<T, Vec<T>>) -> HashSet<&'a T>
where
    T: Eq + Hash,
{
    dfs_with_visitor(start, graph, |_n| {})
}

pub fn dfs_with_visitor<'a, T, F>(
    start: &'a T,
    graph: &'a HashMap<T, Vec<T>>,
    mut visitor: F,
) -> HashSet<&'a T>
where
    T: Eq + Hash + Sized,
    F: FnMut(&'a T),
{
    let mut visited: HashSet<&'a T> = HashSet::new();
    let mut q: VecDeque<&'a T> = VecDeque::new();

    q.push_back(start);
    visited.insert(start);
    visitor(start);

    while q.len() > 0 {
        let cur = q.pop_back().unwrap();
        if !visited.contains(cur) {
            visited.insert(cur);
            visitor(cur);
        }

        if let Some(neighbours) = graph.get(cur) {
            for n in neighbours {
                if !visited.contains(n) {
                    q.push_back(n);
                }
            }
        }
    }

    visited
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_empty() {
        let graph = HashMap::new();
        let visited = bfs(&3, &graph);
        assert_eq!(1, visited.len());
    }

    #[test]
    fn test_bfs_traverse() {
        let mut graph: HashMap<i64, Vec<i64>> = HashMap::new();
        graph.insert(0, vec![1, 2]);
        graph.insert(1, vec![0, 2, 3, 4]);
        graph.insert(2, vec![0, 1, 3, 5]);
        graph.insert(3, vec![1, 2, 4, 5]);
        graph.insert(4, vec![2, 3]);
        graph.insert(5, vec![1, 3]);
        graph.insert(6, vec![7, 8]);
        graph.insert(7, vec![6]);
        graph.insert(8, vec![6]);

        let mut order = vec![];
        let visited = bfs_with_visitor(&0, &graph, |n| order.push(n));
        assert_eq!(6, visited.len());
        assert!(visited.contains(&5));
        assert!(!visited.contains(&6));
        assert_eq!(vec![&0, &1, &2, &3, &4, &5], order);

        order = vec![];
        let visited = bfs_with_visitor(&6, &graph, |n| order.push(n));
        assert_eq!(3, visited.len());
        assert!(visited.contains(&6));
        assert!(!visited.contains(&5));
        assert_eq!(vec![&6, &7, &8], order);
    }

    #[test]
    fn test_dfs_empty() {
        let graph = HashMap::new();
        let visited = dfs(&3, &graph);
        assert_eq!(1, visited.len());
    }

    #[test]
    fn test_dfs_traverse() {
        let mut graph = HashMap::new();
        graph.insert("0", vec!["1", "2"]);
        graph.insert("1", vec!["0", "2", "3", "4"]);
        graph.insert("2", vec!["0", "1", "3", "5"]);
        graph.insert("3", vec!["1", "2", "4", "5"]);
        graph.insert("4", vec!["2", "3"]);
        graph.insert("5", vec!["1", "3"]);
        graph.insert("6", vec!["7", "8"]);
        graph.insert("7", vec!["6"]);
        graph.insert("8", vec!["6"]);

        let mut order = vec![];
        let visited = dfs_with_visitor(&"0", &graph, |n| order.push(n));
        assert_eq!(6, visited.len());
        assert!(visited.contains(&"5"));
        assert!(!visited.contains(&"6"));
        assert_eq!(vec![&"0", &"2", &"5", &"3", &"4", &"1"], order);

        order = vec![];
        let visited = dfs_with_visitor(&"6", &graph, |n| order.push(n));
        assert_eq!(3, visited.len());
        assert!(visited.contains(&"6"));
        assert!(!visited.contains(&"5"));
        assert_eq!(vec![&"6", &"8", &"7"], order);
    }
}
