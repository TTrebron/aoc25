use std::{
    cmp::Reverse,
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    process::ExitCode,
};

use crate::graph::{Edge, Graph, Vertex};

mod graph;

fn process_file<F>(reader: BufReader<File>, mut process_line: F) -> io::Result<()>
where
    F: FnMut(&str),
{
    for line_result in reader.lines() {
        process_line(line_result?.as_str());
    }

    Ok(())
}

fn main() -> ExitCode {
    // filename parameter
    let Some(filename) = env::args().nth(1) else {
        eprintln!(
            "Usage: {} <filename> <connections count>",
            env::args().nth(0).unwrap_or_default()
        );
        return ExitCode::FAILURE;
    };

    // connections count parameter
    let connections = match env::args().nth(2) {
        Some(count_str) => count_str.parse::<usize>().unwrap_or(10),
        None => {
            eprintln!(
                "Usage: {} <filename> <connections count>",
                env::args().nth(0).unwrap_or_default()
            );
            return ExitCode::FAILURE;
        }
    };

    // open file
    let reader = match File::open(filename.as_str()) {
        Ok(file) => BufReader::new(file),
        Err(e) => {
            eprintln!("Error opening {}: {}", filename, e);
            return ExitCode::FAILURE;
        }
    };

    // read vertices
    let mut main_graph = Graph::new();
    if let Err(e) = process_file(reader, |line| {
        let mut words = line.split(',');

        let new_vertex = Vertex::new(
            words
                .next()
                .unwrap_or_default()
                .parse::<u32>()
                .unwrap_or_default(),
            words
                .next()
                .unwrap_or_default()
                .parse::<u32>()
                .unwrap_or_default(),
            words
                .next()
                .unwrap_or_default()
                .parse::<u32>()
                .unwrap_or_default(),
        );
        main_graph.push_vertex(new_vertex);
    }) {
        eprintln!("Error reading file: {}", e);
    };

    if main_graph.vertices_len() == 0 {
        return ExitCode::FAILURE;
    }

    // calculate distances for all possible unique vertex pairs
    let mut distances = vec![];
    for i in 0..main_graph.vertices_len() {
        let v1 = main_graph.get_vertex(i);

        for j in i + 1..main_graph.vertices_len() {
            let v2 = main_graph.get_vertex(j);

            let dist = (((v1.x.abs_diff(v2.x) as u64).pow(2)
                + (v1.y.abs_diff(v2.y) as u64).pow(2)
                + (v1.z.abs_diff(v2.z) as u64).pow(2)) as f64)
                .sqrt();

            distances.push((dist, Edge::new(v1.clone(), v2.clone())));
        }
    }

    // sort distances
    distances.sort_by(|(dist, _), (dist2, _)| dist.total_cmp(dist2));

    // print distances and stats
    for (dist, edge) in distances.iter() {
        println!("Distance between {:?} and {:?}: {}", edge.v1, edge.v2, dist);
    }

    println!(
        "Total vertices: {} - Total connections: {}",
        main_graph.vertices_len(),
        distances.len()
    );

    // connect closest N vertex pairs
    for i in 0..connections {
        main_graph.add_edge(distances[i].1.clone());
    }

    println!("Main graph: {:?}", main_graph);

    // break up the main graph into separate subgraphs
    let mut subgraphs = vec![];
    while main_graph.edges_len() > 0 {
        let start_edge = main_graph.pop_edge();
        let mut subgraph = Graph::new();
        println!("Finding edges in v1 direction...");
        main_graph.move_all_connected_edges_to_other_graph(start_edge.v1.clone(), &mut subgraph);
        println!("Finding edges in v2 direction...");
        main_graph.move_all_connected_edges_to_other_graph(start_edge.v2.clone(), &mut subgraph);
        subgraph.add_edge(start_edge);

        subgraphs.push(subgraph);
    }

    // sort by subgraph size
    subgraphs.sort_by_key(|graph| Reverse(graph.vertices_len()));

    //println!("{:?}", subgraphs);

    if subgraphs.len() < 3 {
        eprintln!("Error, number of separate graphs are less than 3!");
        return ExitCode::FAILURE;
    }

    // calculate solution
    let largest_graph_sizes = subgraphs
        .first_chunk::<3>()
        .unwrap()
        .iter()
        .map(|graph| graph.vertices_len())
        .collect::<Vec<_>>();

    let first_part_solution =
        largest_graph_sizes[0] * largest_graph_sizes[1] * largest_graph_sizes[2];

    println!(
        "The product of the largest three separate graphs' sizes ({}*{}*{}): {}",
        largest_graph_sizes[0], largest_graph_sizes[1], largest_graph_sizes[2], first_part_solution
    );

    ExitCode::SUCCESS
}
