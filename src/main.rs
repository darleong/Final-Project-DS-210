// import important modules and the HashMap data structure from standard library
mod load_edges;
mod graph;
mod bfs;

use std::collections::HashMap;

// function to count distribution of degrees in graph
fn count_degree(result: Vec<i32>) -> Vec<(usize, usize)> {
    let mut count = HashMap::new();
    for degree in result {
        if degree != -1 {
            *count.entry(degree as usize).or_insert(0) += 1;
        }
    }
    let mut count_vec: Vec<(usize, usize)> = count.into_iter().collect();
    count_vec.sort_by_key(|k| k.0);
    count_vec
}


fn main() {
    let read_edges = match load_edges::load_edges("facebook_combined.txt") {
        Ok(edges) => edges,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            return;
        }
    };

    let graph = graph::Graph::create_directed(4039, &read_edges);

    let mut result = Vec::new();

    for i in 0..4039 {
        result.push(bfs::calculate_average_distance(i, &graph));
    }
    // print total nodes, edges and the degree distribution
    println!("Total of 4039 nodes, 88234 edges");
    println!("Degree distribution: {:?}", count_degree(result));

    let min_value = graph.outedges.iter().map(|l| l.len()).min().unwrap();
    let max_value = graph.outedges.iter().map(|l| l.len()).max().unwrap();

    println!("Min Connections: {}", min_value);
    println!("Max Connections: {}", max_value);
}

// test to ensure directed edges are created correctly 
#[test]
fn test_directed_edges() {
    let edges = vec![(0,2),(1,2),(2,4),(2,6)];
    let graph = graph::Graph::create_directed(7, &edges);
    let count_edge: Vec<_> = graph.outedges.iter().map(|j| j.len()).collect();

    assert_eq!(count_edge, vec![0, 0, 2, 0, 0, 0, 0]);
}

#[test]
fn test_count_degree_sum() {
    let count_sample: Vec<i32> = vec![1, 1, 2, 2, 2, 4, 8];
    let count_test = count_degree(count_sample);
    assert_eq!(count_test, vec![(1, 2), (2, 3), (4, 1), (8, 1)]);
}
