use std::{fmt::Debug, rc::Rc};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Vertex {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl Debug for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Vertex {
    pub fn new(x: u32, y: u32, z: u32) -> Vertex {
        Vertex { x: x, y: y, z: z }
    }
}

#[derive(Clone)]
pub struct Edge {
    pub v1: Rc<Vertex>,
    pub v2: Rc<Vertex>,
}

impl<'a> Debug for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}<=>{:?}", self.v1, self.v2)
    }
}

impl<'a> Eq for Edge {}
impl<'a> PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.v1 == other.v1 && self.v2 == other.v2) || (self.v1 == other.v2 && self.v2 == other.v1)
    }
}

impl<'a> Edge {
    pub fn new(v1: Rc<Vertex>, v2: Rc<Vertex>) -> Edge {
        Edge { v1: v1, v2: v2 }
    }
}

#[derive(Debug)]
pub struct Graph {
    vertices: Vec<Rc<Vertex>>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            vertices: vec![],
            edges: vec![],
        }
    }

    // allocate and push vertex
    pub fn push_vertex(&mut self, vertex: Vertex) {
        let vertex_alloc = Rc::new(vertex);
        if !self.vertices.contains(&vertex_alloc) {
            self.vertices.push(vertex_alloc);
        }
    }

    // push already allocated vertex
    pub fn push_vertex_ref(&mut self, vertex: Rc<Vertex>) {
        if !self.vertices.contains(&vertex) {
            self.vertices.push(vertex);
        }
    }

    // get vertices count
    pub fn vertices_len(&self) -> usize {
        self.vertices.len()
    }

    // get a new reference to a vertex
    pub fn get_vertex(&self, i: usize) -> Rc<Vertex> {
        self.vertices[i].clone()
    }

    // add edge
    pub fn add_edge(&mut self, edge: Edge) {
        if !self.edges.contains(&edge) {
            self.edges.push(edge);
        }
    }

    // get edges count
    pub fn edges_len(&self) -> usize {
        self.edges.len()
    }

    // remove an edge
    pub fn pop_edge(&mut self) -> Edge {
        self.remove_edge(0)
    }
    pub fn remove_edge(&mut self, i: usize) -> Edge {
        self.edges.remove(i)
    }

    // recursively move all edges connected to common_vertex to other_graph
    pub fn move_all_connected_edges_to_other_graph(
        &mut self,
        common_vertex: Rc<Vertex>,
        other_graph: &mut Graph,
    ) {
        println!(
            "====== Looking up neighbors for {:?} =======",
            common_vertex
        );
        other_graph.push_vertex_ref(common_vertex.clone());

        // collect and remove all edges connected to common_vertex
        let mut neighbors = self
            .edges
            .extract_if(0..self.edges_len(), |edge| {
                edge.v1 == common_vertex || edge.v2 == common_vertex
            })
            .collect::<Vec<_>>();

        // print all edges that will be removed
        for neighbor_edge in neighbors.iter() {
            println!("Neighbor found: {:?}", neighbor_edge);
        }

        // if there are any edges connected, call this function on them too
        for neighbor_edge in neighbors.iter() {
            if neighbor_edge.v1 == common_vertex {
                let other_vertex = neighbor_edge.v2.clone();
                self.move_all_connected_edges_to_other_graph(other_vertex, other_graph);
            } else if neighbor_edge.v2 == common_vertex {
                let other_vertex = neighbor_edge.v1.clone();
                self.move_all_connected_edges_to_other_graph(other_vertex, other_graph);
            }
        }

        // add all removed edges to other_graph
        while neighbors.len() > 0 {
            other_graph.add_edge(neighbors.pop().unwrap());
        }
    }
}
