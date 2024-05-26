use crate::gfa::Orientation;
use std::collections::{HashMap, HashSet};


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

// loop through the path (segment, orientation) pairs, and keep track of the visited segments, if you find a segment that has already been visited, then there is a cycle. Print to terminal the subpath that starts and ends at the cycle (so with the same segment and orientation at the start and end)
pub fn path_has_cycle(path: &Vec<(String, Orientation)>) -> bool {
    let mut visited = HashSet::new(); // (segment, orientation) 
    // loop using idex so we can get the subpath that forms the cycle
    for i in 0..path.len() {
        let (segment, orientation) = &path[i];
        if visited.contains(&(segment.clone(), orientation.clone())) {
            println!("Cycle detected at segment: {}, orientation: {:?}", segment, orientation);
            let mut subpath = Vec::new(); // (segment, orientation)
            subpath.push((segment.clone(), orientation.clone()));
            for j in (0..i).rev() {
                let (subsegment, suborientation) = &path[j];
                if subsegment == segment && suborientation == orientation {
                    subpath.push((subsegment.clone(), suborientation.clone()));
                    break;
                } else {
                    subpath.push((subsegment.clone(), suborientation.clone()));
                }
            }
            println!("Subpath forming cycle: {:?}", subpath);
            return true;
        } else {
            visited.insert((segment.clone(), orientation.clone()));
        }
    }
    false
}
