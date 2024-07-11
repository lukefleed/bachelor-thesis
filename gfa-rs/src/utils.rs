use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use crate::{gfa::Orientation, graph::AdjacencyGraph};

pub fn change_and_replace(
    id: &usize,
    orient: &Orientation,
    sequence_map_sorted: &mut HashMap<usize, String>,
    graph: &mut AdjacencyGraph<(usize, Orientation)>,
) {
    // If the orientation is forward, return
    if orient == &Orientation::Forward {
        return;
    }

    // Create a new id by incrementing the last id
    let new_id = *sequence_map_sorted.keys().last().unwrap() + 1;

    // Extract the value of the element with key id in the sequence_map_sorted
    let value = sequence_map_sorted.get(id).unwrap().to_owned();

    // Insert the new id and the value in the sequence_map_sorted
    sequence_map_sorted.insert(new_id, value);

    // Collect edges to modify before mutating the graph
    let edges_to_modify: Vec<((usize, Orientation), (usize, Orientation))> = graph
        .edges()
        .filter_map(|(from, to)| {
            if from.0 == *id && to.1 == Orientation::Reverse {
                Some((from.clone(), to.clone()))
            } else if to.0 == *id && from.1 == Orientation::Reverse {
                Some((from.clone(), to.clone()))
            } else {
                None
            }
        })
        .collect();

    // Now modify the graph based on collected edges
    for (from, to) in edges_to_modify {
        graph.remove_edge(&from, &to);
        if from.0 == *id && to.1 == Orientation::Reverse {
            graph.add_edge((new_id, Orientation::Forward), to);
        } else if to.0 == *id && from.1 == Orientation::Reverse {
            graph.add_edge(from, (new_id, Orientation::Forward));
        }
    }
}

pub fn write_graph_to_file(
    graph: &AdjacencyGraph<(usize, Orientation)>,
    segments: &HashMap<usize, String>,
    file: &mut File,
) -> std::io::Result<()> {
    // Write the number of nodes
    let number_of_nodes = graph.nodes().into_iter().count();
    writeln!(file, "{}", number_of_nodes)?;

    // Write the nodes
    for (id, sequence) in segments {
        writeln!(file, "{}\t{}", id, sequence)?;
    }

    // Write the edges
    for (from, to) in graph.edges() {
        writeln!(file, "{}\t{}", from.0, to.0)?;
    }

    Ok(())
}
