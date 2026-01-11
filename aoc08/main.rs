use std::{
    cmp::Reverse,
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    ops::Add,
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

#[derive(PartialEq)]
enum AoCTask {
    A,
    B,
}

fn add_edge_and_connect_subgraphs(subgraphs: &mut Vec<Graph>, current_edge: Edge) {
    let v1_index = subgraphs
        .iter_mut()
        .position(|graph| graph.has_vertex(&current_edge.v1));

    let v2_index = subgraphs
        .iter_mut()
        .position(|graph| graph.has_vertex(&current_edge.v2));

    match (v1_index, v2_index) {
        (None, None) => {
            println!(
                "Edge {:?} not found in any subgraphs -> creating new one",
                current_edge
            );
            let mut new_graph = Graph::new();
            new_graph.push_vertex_ref(current_edge.v1.clone());
            new_graph.push_vertex_ref(current_edge.v2.clone());
            new_graph.add_edge(current_edge);
            subgraphs.push(new_graph);
        }

        (Some(v1_index), None) => {
            println!(
                "Edge {:?}: v1 found in graph {} -> adding other to graph",
                current_edge,
                subgraphs[v1_index].id()
            );
            subgraphs[v1_index].push_vertex_ref(current_edge.v2.clone());
            subgraphs[v1_index].add_edge(current_edge);
            println!(
                "Graph {} new size: {} ",
                subgraphs[v1_index].id(),
                subgraphs[v1_index].print_stats()
            )
        }

        (None, Some(v2_index)) => {
            println!(
                "Edge {:?}: v2 found in graph {} -> adding other to graph",
                current_edge,
                subgraphs[v2_index].id()
            );

            subgraphs[v2_index].push_vertex_ref(current_edge.v1.clone());
            subgraphs[v2_index].add_edge(current_edge);
            println!(
                "Graph {} new size: {} ",
                subgraphs[v2_index].id(),
                subgraphs[v2_index].print_stats()
            )
        }

        (Some(v1_index), Some(v2_index)) => {
            if v1_index == v2_index {
                println!(
                    "Edge {:?}: both vertices found in graph {} -> adding edge only",
                    current_edge,
                    subgraphs[v1_index].id()
                );
                subgraphs[v1_index].add_edge(current_edge);
                println!(
                    "Graph {} new size: {} ",
                    subgraphs[v1_index].id(),
                    subgraphs[v1_index].print_stats()
                )
            } else {
                let (first_index, second_index) = order_pair((v1_index, v2_index));
                println!(
                    "Edge {:?}: vertices found in two separate graphs -> merging {} into {}",
                    current_edge,
                    subgraphs[second_index].id(),
                    subgraphs[first_index].id()
                );

                let split_index = first_index.add(1).min(subgraphs.len() - 1);
                let (first_slice, second_slice) = subgraphs.split_at_mut(split_index);
                first_slice[first_index].absorb(&mut second_slice[second_index - split_index]);
                let removed_id = subgraphs.remove(second_index).id();

                println!(
                    "Graph {} new size: {} ",
                    subgraphs[first_index].id(),
                    subgraphs[first_index].print_stats()
                );
                println!("Graph {} removed", removed_id);
            }
        }
    }
}

fn order_pair<T: Ord + Copy>((a, b): (T, T)) -> (T, T) {
    (a.min(b), a.max(b))
}

fn main() -> ExitCode {
    // filename parameter
    let Some(filename) = env::args().nth(1) else {
        eprintln!(
            "Usage: {} <filename> <<a> <connections count> | <b>>",
            env::args().nth(0).unwrap_or_default()
        );
        return ExitCode::FAILURE;
    };

    // part to solve - a or b
    let task = match env::args().nth(2).map(|task_str| {
        task_str
            .chars()
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase()
    }) {
        Some('a') => AoCTask::A,
        Some('b') => AoCTask::B,
        _ => {
            eprintln!(
                "Usage: {} <filename> <<a> <connections count> | <b>>",
                env::args().nth(0).unwrap_or_default()
            );
            return ExitCode::FAILURE;
        }
    };

    // connections count parameter
    let connections = match env::args().nth(3) {
        Some(count_str) => count_str.parse::<usize>().unwrap_or(10),
        None => {
            if task != AoCTask::B {
                eprintln!(
                    "Usage: {} <filename> <<a> <connections count> | <b>>",
                    env::args().nth(0).unwrap_or_default()
                );
                return ExitCode::FAILURE;
            }
            0
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

    let global_vertices = main_graph.vertices_len();

    println!(
        "Total vertices: {} - Total connections: {}",
        main_graph.vertices_len(),
        distances.len()
    );

    if task == AoCTask::A {
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
            main_graph
                .move_all_connected_edges_to_other_graph(start_edge.v1.clone(), &mut subgraph);
            println!("Finding edges in v2 direction...");
            main_graph
                .move_all_connected_edges_to_other_graph(start_edge.v2.clone(), &mut subgraph);
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
            largest_graph_sizes[0],
            largest_graph_sizes[1],
            largest_graph_sizes[2],
            first_part_solution
        );
    } else if task == AoCTask::B {
        let mut subgraphs: Vec<Graph> = vec![];

        distances.reverse();

        // add edges until at least two separate graphs are created
        while subgraphs.len() < 2 {
            let current_edge = match distances.pop() {
                Some(conn) => conn.1,
                None => {
                    eprintln!("Error: Second graph never created in this order");
                    return ExitCode::FAILURE;
                }
            };

            add_edge_and_connect_subgraphs(&mut subgraphs, current_edge);
        }

        println!("We now have at least 2 subgraphs. Trying to connect them...");

        // add more edges until there's only one subgraph and all vertices are connected
        while subgraphs.len() > 1
            || subgraphs
                .iter()
                .fold(0, |max, graph| max.max(graph.vertices_len()))
                < global_vertices
        {
            let current_edge = match distances.pop() {
                Some(conn) => conn.1,
                None => {
                    eprintln!("Error: Ran out of edges");
                    return ExitCode::FAILURE;
                }
            };

            add_edge_and_connect_subgraphs(&mut subgraphs, current_edge);
        }
    }

    ExitCode::SUCCESS
}
