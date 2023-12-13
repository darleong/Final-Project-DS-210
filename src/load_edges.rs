use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;

// reads edge data from a file at the given path and returns it as a vector of tuple pairs.
pub fn load_edges<P: AsRef<Path>>(filepath: P) -> io::Result<Vec<(usize, usize)>> {
    let file_handle = File::open(filepath)?;
    let buffered = BufReader::new(file_handle);

    let mut edges = Vec::new();

    for line in buffered.lines() {
        let current_line = line?;
        // splitting the line by whitespace 
        let nodes: Vec<&str> = current_line.split_whitespace().collect();

        if nodes.len() != 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Line does not contain exactly two vertices"));
        }

        let node1 = nodes[0].parse::<usize>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse the first vertex"))?;
        let node2 = nodes[1].parse::<usize>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse the second vertex"))?;

        edges.push((node1, node2));
    }

    Ok(edges)
}
