use std::collections::HashMap;

use argh::FromArgs;
use gfa::{Entry, Orientation};
use graph::AdjacencyGraph;

mod gfa;
mod graph;
mod parser;
mod paths;

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
            let mut graph: AdjacencyGraph<(String, Orientation)> = AdjacencyGraph::new();
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

            println!(
                "\nBEFORE CONVERSION\nGraph has {} edges and {} nodes",
                graph.edges().count(),
                graph.adjacencies().len()
            );

            if graph.has_cycle() {
                println!("Cycle(s) detected in the graph");
            } else {
                println!("No cycle(s) detected in the graph");
            }

            // Convert the graph to DAG
            let dag = graph.to_dag();

            println!(
                "\nAFTER CONVERSION\nDAG has {} edges and {} nodes",
                dag.edges().count(),
                dag.adjacencies().len()
            );

            // add an if else print for cycles
            if dag.has_cycle() {
                println!("Cycle(s) detected in the DAG");
            } else {
                println!("No cycle(s) detected in the DAG");
            }

            // for (name, path) in paths.iter() {
            //     println!("\nPath name: {}", name);
            //     if paths::path_has_cycle(path) {
            //         println!("Cycle detected in path: {}", name);
            //         // count by segment and orientation the number of times they appear in the path
            //         // let mut segment_orient_count = HashMap::new();
            //         // for (segment, orient) in path {
            //         //     let count = segment_orient_count
            //         //         .entry((segment.clone(), orient.clone()))
            //         //         .or_insert(0);
            //         //     *count += 1;
            //         // }
            //         // // print the segment and orientation that appear more than once
            //         // for ((segment, orient), count) in segment_orient_count.iter() {
            //         //     if *count > 1 {
            //         //         println!(
            //         //             "Segment: {}, Orientation: {:?}, Count: {}",
            //         //             segment, orient, count
            //         //         );
            //         //     }
            //         // }
            //     }
            // }

            // Print the path gi|568815567:3779003-3792415

            // let the_path = paths.iter().find(|(name, _)| name.as_str()             == "gi|568815567:3779003-3792415");

            // println!("Path: {:?}", the_path);

            // Print the graph
            // for ((from, orient), adjacencies) in graph.adjacencies().iter() {
            //     println!(
            //         "{}{} -> {}",
            //         from,
            //         orient,
            //         adjacencies
            //             .iter()
            //             .map(|(to, orient)| format!("{}{}", to, orient))
            //             .collect::<Vec<String>>()
            //             .join(", ")
            //     );
            // }

            // let cc = graph.compute_ccs();

            // println!("CCs: {:?}", cc);
            // println!("Number of connected components: {}", cc.len());
        }
    }

    Ok(())
}
