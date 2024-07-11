use std::collections::HashMap;

use argh::FromArgs;
use gfa::{Entry, Orientation};
use graph::AdjacencyGraph;

mod gfa;
mod graph;
mod parser;
mod paths;
mod utils;

#[derive(FromArgs, PartialEq, Debug)]
/// A tool to work with GFA files
struct CliTool {
    #[argh(subcommand)]
    nested: MySubCommandEnum,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum MySubCommandEnum {
    Show(CommandConvert),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Parse and show the content of a file
#[argh(subcommand, name = "convert")]
struct CommandConvert {
    #[argh(option, short = 'i')]
    /// file to read
    input: String,

    #[argh(option, short = 'o')]
    /// file to write
    output: String,
}

fn main() -> std::io::Result<()> {
    let opts = argh::from_env::<CliTool>();

    match opts.nested {
        MySubCommandEnum::Show(show) => {
            let file = std::fs::File::open(show.input)?;
            let entries = parser::parse_source(file)?;

            let mut sequence_map = HashMap::new();
            let mut graph: AdjacencyGraph<(usize, Orientation)> = AdjacencyGraph::new();
            let mut paths: HashMap<String, Vec<(String, Orientation)>> = HashMap::new();

            for entry in entries {
                match entry {
                    Entry::Segment { id, sequence } => {
                        sequence_map.insert(id.clone(), sequence);
                    }
                    Entry::Link {
                        from,
                        from_orient,
                        to,
                        to_orient,
                    } => {
                        graph.add_edge((from.clone(), from_orient), (to.clone(), to_orient));
                    }
                    Entry::Path { name, segments } => {
                        let mut path = Vec::new();
                        for (segment, orient) in segments {
                            path.push((segment, orient));
                        }
                        paths.insert(name, path);
                    }
                    _ => {}
                }
            }

            // print information of the graph before removing converting it to a dag
            println!("Graph before removing isolated nodes:");
            println!("Number of nodes: {:?}", graph.nodes().len());
            println!("Number of edges: {:?}", graph.edges().count());
            println!("Has Cycles? {:?}", graph.has_cycle());

            let mut dag = graph.to_dag();

            println!("\nGraph after removing isolated nodes:");
            println!("Number of nodes: {:?}", dag.nodes().len());
            println!("Number of edges: {:?}", dag.edges().count());
            println!("Has Cycles? {:?}", dag.has_cycle());

            // remove from the dag the isolated nodes
            println!("\nRemoving isolated nodes from the graph...");
            let num = dag.nodes().len();
            dag.remove_isolated_nodes();
            if num != dag.nodes().len() {
                println!("Isolated nodes removed.");
            } else {
                println!("No isolated nodes found.");
            }

            println!("Starting to identify keys to remove...");
            // remove from the sequence map the nodes that are not in the dag
            let mut keys_to_remove = Vec::new();

            // loop over the sequence map, if there is a key that does not match any node id in the dag, add it to the keys_to_remove.
            for key in sequence_map.keys() {
                let mut found = false;
                for node in dag.nodes() {
                    if node.0 == *key {
                        found = true;
                        break;
                    }
                }
                if !found {
                    keys_to_remove.push(key.clone());
                }
            }

            println!("Keys to remove identified: {}", keys_to_remove.len());

            // remove the keys from the sequence map
            for key in keys_to_remove {
                sequence_map.remove(&key);
            }

            // Sort the sequence map
            println!("Sorting the sequence map...");
            let mut keys = sequence_map.keys().cloned().collect::<Vec<usize>>();
            keys.sort();
            let mut sequence_map_sorted = HashMap::new();
            for key in keys {
                sequence_map_sorted.insert(key, sequence_map.get(&key).unwrap().to_string());
            }

            // Step 1: Collect node identifiers into a new vector to avoid borrowing issues
            println!("Collecting node identifiers...");
            let node_ids: Vec<_> = dag
                .nodes()
                .into_iter()
                .map(|node| (node.0, node.1.clone()))
                .collect();

            // Step 2: Iterate over the new collection of node identifiers
            println!("Applying changes to the graph...");
            for (node_id_0, node_id_1) in node_ids {
                // Step 3: Apply changes to `graph` using the mutable borrow
                utils::change_and_replace(
                    &node_id_0,
                    &node_id_1,
                    &mut sequence_map_sorted,
                    &mut dag,
                );
            }

            println!("Changes applied.");

            // Write the graph to a file
            println!("Writing the graph to a file...");
            let mut file = std::fs::File::create(show.output)?;
            utils::write_graph_to_file(&dag, &sequence_map_sorted, &mut file)?;

            println!("Graph written to file successfully.");

            Ok(())
        }
    }
}
