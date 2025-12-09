use petgraph::{Graph, Undirected, unionfind::UnionFind};
use std::collections::HashMap;

advent_of_code::solution!(8);

fn create_graph(input: &str) -> Graph<(f64, f64, f64), (), Undirected> {
    let mut graph: Graph<(f64, f64, f64), (), Undirected> = Graph::new_undirected();

    input.trim().lines().for_each(|line| {
        let mut nums = line.split(',');

        let x: f64 = nums.next().unwrap().parse().unwrap();
        let y: f64 = nums.next().unwrap().parse().unwrap();
        let z: f64 = nums.next().unwrap().parse().unwrap();

        graph.add_node((x, y, z));
    });

    graph
}

fn calculate_distances(
    graph: &Graph<(f64, f64, f64), (), Undirected>,
) -> Vec<(f64, petgraph::graph::NodeIndex, petgraph::graph::NodeIndex)> {
    let mut distances = Vec::new();

    for i in graph.node_indices() {
        let (x1, y1, z1) = graph[i];

        for j in graph.node_indices() {
            if j >= i {
                continue;
            }

            let (x2, y2, z2) = graph[j];

            let dx = x2 - x1;
            let dy = y2 - y1;
            let dz = z2 - z1;
            let distance = (dx * dx + dy * dy + dz * dz).sqrt();

            distances.push((distance, i, j));
        }
    }

    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    distances
}

fn solve(input: &str, num_connections: usize) -> Option<u64> {
    let mut graph = create_graph(input);
    let distances = calculate_distances(&graph);

    let mut uf = UnionFind::new(graph.node_count());

    for (_, node_i, node_j) in distances.iter().take(num_connections) {
        graph.add_edge(*node_i, *node_j, ());
        uf.union(node_i.index(), node_j.index());
    }

    let mut component_sizes: HashMap<usize, usize> = HashMap::new();

    for node in graph.node_indices() {
        let root = uf.find(node.index());
        *component_sizes.entry(root).or_insert(0) += 1;
    }

    // Get the three largest component sizes
    let mut sizes: Vec<usize> = component_sizes.values().copied().collect();
    sizes.sort_by(|a, b| b.cmp(a)); // Sort descending

    let result = sizes.iter().take(3).product::<usize>();

    Some(result as u64)
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 1000)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut graph = create_graph(input);
    let distances = calculate_distances(&graph);

    let mut uf = UnionFind::new(graph.node_count());
    let mut num_edges_added = 0;

    for (_, node_i, node_j) in distances.iter() {
        graph.add_edge(*node_i, *node_j, ());

        let root_i = uf.find(node_i.index());
        let root_j = uf.find(node_j.index());

        if root_i != root_j {
            uf.union(node_i.index(), node_j.index());
            num_edges_added += 1;

            if num_edges_added == graph.node_count() - 1 {
                return Some((graph[*node_i].0 * graph[*node_j].0) as u64);
            }
        }
    }

    Some(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
