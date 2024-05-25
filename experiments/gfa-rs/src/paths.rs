use crate::gfa::Orientation;
use std::collections::HashMap;

// Print the paths
pub fn _print_paths(paths: &HashMap<String, Vec<(String, Orientation)>>) {
    for (name, path) in paths {
        println!("Path name: {}", name);
        for (segment, orientation) in path {
            println!("Segment: {}, Orientation: {:?}", segment, orientation);
        }
        println!("\n");
    }
}

// Check if the paths have a cycle
pub fn paths_have_cycle(paths: &HashMap<String, Vec<(String, Orientation)>>) {
    println!("Number of paths in the GFA: {}", paths.len());

    paths.iter().for_each(|(name, path)| {
        println!("Path name: {}", name);
        println!("\tPath length: {}", path.len());

        // check if the path has a cycle without converting it to a graph
        let mut visited = HashMap::new();
        let mut parent = HashMap::new();
        let mut has_cycle = false;
        let mut cycle_start = String::new();
        let mut cycle_path = Vec::new();

        for (segment, orientation) in path {
            if visited.contains_key(segment) {
                has_cycle = true;
                cycle_start = segment.clone();
                break;
            }
            visited.insert(segment.clone(), orientation.clone());
            parent.insert(segment.clone(), (segment.clone(), orientation.clone()));
        }

        if has_cycle {
            // reconstruct the cycle path
            let mut current = cycle_start.clone();
            while let Some((seg, _)) = parent.get(&current) {
                cycle_path.push(seg.clone());
                if &current == &cycle_start && cycle_path.len() > 1 {
                    break;
                }
                current = seg.clone();
            }
            println!("\tHas cycle: true");
            println!("\tCycle path: {:?}", cycle_path);
        } else {
            println!("\tHas cycle: false");
        }

        println!("\n");
    });
}
