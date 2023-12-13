use std::collections::VecDeque;
use crate::graph::Graph;

pub fn calculate_average_distance(start: usize, graph: &Graph) -> i32 {
    // initialize  vector to track distances from  start 
    let mut distances = vec![None; graph.n];
    distances[start] = Some(0);

    let mut node_queue = VecDeque::new();
    node_queue.push_back(start);

    let mut accumulated_distance = 0u32;
    let mut nodes_counted = 0u32;

    // bfs loop
    while let Some(current_node) = node_queue.pop_front() {
        if let Some(current_distance) = distances[current_node] {
            for &adjacent_node in &graph.outedges[current_node] {
                if distances[adjacent_node].is_none() {
                    distances[adjacent_node] = Some(current_distance + 1);
                    node_queue.push_back(adjacent_node);
                    accumulated_distance += current_distance + 1;
                    nodes_counted += 1;
                }
            }
        }
    }

    // returning -1 if no nodes are reachable
    if nodes_counted == 0 {
        -1
    } else {
        (accumulated_distance as i32) / (nodes_counted as i32)
    }
}
