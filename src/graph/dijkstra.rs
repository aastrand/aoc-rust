use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub fn dijkstra<'a, T>(
    start: &'a T,
    end: &'a T,
    graph: &'a HashMap<T, Vec<T>>,
) -> (Vec<&'a T>, HashMap<&'a T, f64>, HashMap<&'a T, &'a T>)
where
    T: Eq + Hash,
{
    let mut q: HashSet<&'a T> = HashSet::new();
    let mut dist: HashMap<&'a T, f64> = HashMap::new();
    let mut prev: HashMap<&'a T, &'a T> = HashMap::new();

    for pos in graph.keys() {
        dist.insert(pos, f64::INFINITY);
        q.insert(pos);
    }
    dist.insert(start, 0.0);

    while q.len() > 0 {
        if let Some(u) = get_min_dist(&q, &dist) {
            q.remove(u);

            if let Some(neighbours) = graph.get(u) {
                for n in neighbours {
                    let alt = dist.get(u).unwrap_or(&f64::INFINITY) + 1.0;
                    if alt < *dist.get(n).unwrap_or(&f64::INFINITY) {
                        dist.insert(n, alt);
                        prev.insert(n, u);
                    }
                }
            }
        }
    }

    let mut path = vec![];
    let mut u = end;
    if prev.contains_key(u) || u == start {
        loop {
            path.insert(0, u);
            if let Some(next) = prev.get(u) {
                u = next;
            } else {
                break;
            }
        }
    }

    (path, dist, prev)
}

fn get_min_dist<'a, T>(q: &HashSet<&'a T>, dist: &HashMap<&'a T, f64>) -> Option<&'a T>
where
    T: Eq + Hash,
{
    let mut min = f64::INFINITY;
    let mut min_node = None;
    for v in q {
        if let Some(d) = dist.get(*v) {
            if *d <= min {
                min = *d;
                min_node = Some(*v);
            }
        }
    }

    min_node
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra_empty() {
        let graph = HashMap::new();

        let (path, dist, prev) = dijkstra(&0, &10, &graph);
        assert_eq!(0, path.len());
        assert_eq!(1, dist.len());
        assert_eq!(0, prev.len());
    }

    #[test]
    fn test_dijkstra() {
        let mut graph = HashMap::new();
        graph.insert(0, vec![1, 5]);
        graph.insert(1, vec![0, 2]);
        graph.insert(2, vec![1, 3, 4]);
        graph.insert(3, vec![2, 4]);
        graph.insert(4, vec![2, 10]);
        graph.insert(5, vec![0, 6, 7]);
        graph.insert(6, vec![5, 7, 8, 10]);
        graph.insert(7, vec![5, 6, 8]);
        graph.insert(8, vec![6, 7, 9]);
        graph.insert(9, vec![8]);
        graph.insert(10, vec![4, 6]);

        let (path, dist, prev) = dijkstra(&0, &10, &graph);
        assert_eq!(vec![&0, &5, &6, &10], path);
        assert_eq!(Some(&&6), prev.get(&10));
        assert_eq!(Some(&2.0), dist.get(&6));
    }
}
