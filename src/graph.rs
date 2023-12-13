// basic types for clarity and reusability define
type Vertex = usize;
type ListOfEdges = Vec<(Vertex, Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;

#[derive(Debug)]
pub struct Graph {
    // number of vertices in the graph
    pub n: usize,
    // adjacency lists to store outgoing edges for each vertex
    pub outedges: AdjacencyLists,
}

impl Graph {
    pub fn add_directed_edges(&mut self, edges: &ListOfEdges) {
        for &(u, v) in edges {
            if u < self.n {
                self.outedges[u].push(v);
            }
            // additional error handling or logging can be added here 
        }
    }

    // sorts  adjacency lists for each vertex
    // this can be helpful for algorithms that require ordered adjacency lists
    pub fn sort_graph_lists(&mut self) {
        for adjacency_list in self.outedges.iter_mut() {
            adjacency_list.sort_unstable(); // Using sort_unstable for potentially better performance
        }
    }

    // create  new directed graph given the number of vertices and a list of edges
    pub fn create_directed(n: usize, edges: &ListOfEdges) -> Graph {
        let mut g = Graph {
            n,
            outedges: vec![vec![]; n],
        };
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g
    }
}
