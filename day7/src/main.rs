use std::collections::HashMap;
use std::io::{self, Read};

struct Graph {
    vertices: HashMap<String, usize>,
    edges: Vec<Vec<bool>>,
}

impl Graph {

    // Build a matrix of bools that represent edges and a hashmap for vertices that maps strings
    // to indices in the edge matrix
    pub fn new(edges: &[(String, String)], vertices_keys: &[String]) -> Graph {
        let mut vertices: HashMap<String, usize> = HashMap::with_capacity(vertices_keys.len());
        for i in 0..vertices_keys.len() {
            vertices.insert(vertices_keys[i].clone(), i);
        }

        let mut edges_matrix = Vec::with_capacity(vertices_keys.len());
        for i in 0..vertices_keys.len() {
            edges_matrix.push(Vec::with_capacity(vertices_keys.len()));
            for _j in 0..vertices_keys.len() {
                edges_matrix[i].push(false);
            }
        }

        for (from, to) in edges {
            let from_key = vertices.get(from).unwrap();
            let to_key = vertices.get(to).unwrap();
            edges_matrix[*from_key][*to_key] = true;
        }

        Graph {
            vertices,
            edges: edges_matrix,
        }
    }

    // Set all connections from this key to other nodes to be false
    pub fn remove(&mut self, key: &str) {
        let from_key = self.vertices.get(key).unwrap();
        for i in 0..self.edges.len() {
            self.edges[*from_key][i] = false;
        }
    }

    // Return true if the key is a root node, false otherwise
    pub fn is_root(&self, key: &str) -> bool {
        let to_key = self.vertices.get(key).unwrap();
        for i in 0..self.edges.len() {
            if self.edges[i][*to_key] {
                return false;
            }
        }
        true
    }
}

fn main() -> io::Result<()> {
    // Get our data from stdin
    let mut input: String = String::new();
    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();
    stdin_handle.read_to_string(&mut input)?;

    let (mut vertices, edges) = get_graph_data(input);

    let mut order: Vec<String> = Vec::new();

    let mut graph = Graph::new(&edges, &vertices);

    while vertices.len() > 0 {
        // For each unused vertex, check if it's a root node, if it is, add it to our ouput
        let tmp_v = vertices.clone();
        for (i, s) in tmp_v.iter().enumerate() {
            let tmp = graph.is_root(s);
            if tmp == true {
                graph.remove(s);
                vertices.remove(i);
                order.push(s.to_string());
                break;
            }
        }
    }

    for i in order {
        print!("{}", i);
    }
    println!("");

    Ok(())
}

fn get_graph_data(input: String) -> (Vec<String>, Vec<(String, String)>) {
    let split_input: Vec<&str> = input.split("\n").collect();
    let mut vertices: Vec<String> = Vec::new();
    let mut edges: Vec<(String, String)> = Vec::new();

    for i in &split_input {
        let split_i: Vec<&str> = i.split_whitespace().collect();
        if split_i.len() == 0 {
            continue;
        }
        edges.push((split_i[1].to_string(), split_i[7].to_string()));
        vertices.push(split_i[1].to_string());
        vertices.push(split_i[7].to_string());
    }

    vertices.sort_unstable();
    vertices.dedup();

    (vertices, edges)
}
