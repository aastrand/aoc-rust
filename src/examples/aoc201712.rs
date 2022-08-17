use super::super::collections::hashset_pop;
use super::super::graph::bfs;
use super::super::io::lines_from_file;
use super::super::parse::ints;
use std::collections::{HashMap, HashSet};

fn build_graph(lines: &Vec<String>) -> HashMap<i64, Vec<i64>> {
    let mut graph: HashMap<i64, Vec<i64>> = HashMap::new();
    for line in lines {
        let parts = ints(line);
        graph.insert(parts[0], parts[1..].to_vec());
    }

    graph
}

#[allow(dead_code)]
fn part1(filename: &str) -> i64 {
    let graph = build_graph(&lines_from_file(filename));
    let visited = bfs(&0, &graph);

    visited.len() as i64
}

#[allow(dead_code)]
fn part2(filename: &str) -> i64 {
    let graph = build_graph(&lines_from_file(filename));

    let mut nodes: HashSet<&i64> = graph.keys().collect();
    let mut groups = HashSet::new();
    while nodes.len() > 0 {
        let n = hashset_pop(&mut nodes).unwrap();
        let visited: HashSet<&i64> = bfs(n, &graph);

        let mut group = visited.iter().collect::<Vec<&&i64>>();
        group.sort();
        groups.insert(format!("{:?}", group));

        for n in visited {
            nodes.remove(n);
        }
    }

    groups.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(6, part1("src/examples/aoc201712example.txt"));
        assert_eq!(378, part1("src/examples/aoc201712input.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2, part2("src/examples/aoc201712example.txt"));
        assert_eq!(204, part2("src/examples/aoc201712input.txt"));
    }
}
