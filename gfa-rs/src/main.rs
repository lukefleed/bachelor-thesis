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
    Show(CommandShow),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Parse and show the content of a file
#[argh(subcommand, name = "show")]
struct CommandShow {
    #[argh(option, short = 'i')]
    /// file to read
    input: String,
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

            // Sort the sequence map
            let mut keys = sequence_map.keys().cloned().collect::<Vec<usize>>();
            keys.sort();
            let mut sequence_map_sorted = HashMap::new();
            for key in keys {
                sequence_map_sorted.insert(key, sequence_map.get(&key).unwrap().to_string());
            }

            // Step 1: Collect node identifiers into a new vector to avoid borrowing issues
            let node_ids: Vec<_> = graph
                .nodes()
                .into_iter()
                .map(|node| (node.0, node.1.clone()))
                .collect();

            // Step 2: Iterate over the new collection of node identifiers
            for (node_id_0, node_id_1) in node_ids {
                // Step 3: Apply changes to `graph` using the mutable borrow
                utils::change_and_replace(
                    &node_id_0,
                    &node_id_1,
                    &mut sequence_map_sorted,
                    &mut graph,
                );
            }

            // Write the graph to a file
            let mut file = std::fs::File::create("chrY.pan.fa.a2fb268.4030258.6a1ecc2.smooth.tsv")?;
            utils::write_graph_to_file(&graph, &sequence_map_sorted, &mut file)?;

            Ok(())
        }
    }
}
