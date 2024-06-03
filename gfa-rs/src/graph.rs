use std::{
    collections::{BTreeSet, HashMap, VecDeque},
    fmt::Debug,
    hash::Hash,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)] // Add Clone, Copy, PartialEq, Eq for enum usage
pub enum EdgeType {
    Tree,
    Forward,
    Back,
    Cross,
}

#[derive(Debug)]
pub struct AdjacencyGraph<V>
where
    V: Hash + Eq + Clone,
{
    nodes: BTreeSet<V>,
    adjacencies: HashMap<V, BTreeSet<V>>,
}

#[allow(dead_code)]
impl<V> AdjacencyGraph<V>
where
    V: Hash + Eq + Clone + Debug + Ord,
{
    pub fn new() -> Self {
        AdjacencyGraph {
            nodes: BTreeSet::new(),
            adjacencies: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: V) {
        self.nodes.insert(node);
    }

    pub fn add_edge(&mut self, from: V, to: V) {
        self.add_node(from.clone());
        self.add_node(to.clone());

        self.adjacencies
            .entry(from)
            .or_insert_with(BTreeSet::new)
            .insert(to);
    }

    pub fn get_adjacencies(&self, node: &V) -> Option<&BTreeSet<V>> {
        self.adjacencies.get(node)
    }

    pub fn adjacencies(&self) -> &HashMap<V, BTreeSet<V>> {
        &self.adjacencies
    }

    pub fn edges(&self) -> impl Iterator<Item = (&V, &V)> + '_ {
        self.adjacencies
            .iter()
            .flat_map(|(from, tos)| tos.iter().map(move |to| (from, to)))
    }

    pub fn has_edge(&self, from: &V, to: &V) -> bool {
        if let Some(adjacencies) = self.get_adjacencies(from) {
            adjacencies.contains(&to.to_owned())
        } else {
            false
        }
    }

    pub fn dfs<'a>(&'a self, node: &'a V) -> impl Iterator<Item = V> + 'a {
        let mut visited = BTreeSet::new();
        let mut stack = VecDeque::from([node]);

        std::iter::from_fn(move || {
            while let Some(node) = stack.pop_back() {
                if !visited.insert(node.clone()) {
                    continue; // Already visited
                }

                if let Some(adjacencies) = self.get_adjacencies(node) {
                    stack.extend(adjacencies); // Extend is more efficient than multiple pushes
                }

                return Some(node.clone());
            }
            None // Iteration is complete when the stack is empty
        })
    }

    // Modified DFS for edge classification
    pub fn dfs_classify_edges<'a>(&'a self, start_node: &'a V) -> HashMap<(V, V), EdgeType> {
        let mut visited = BTreeSet::new();
        let mut stack = VecDeque::from([start_node]);
        let mut arrival_times = HashMap::new();
        let mut departure_times = HashMap::new();
        let mut time = 0;
        let mut edge_types = HashMap::new();

        while let Some(node) = stack.pop_back() {
            if !visited.contains(node) {
                visited.insert(node.clone());
                arrival_times.insert(node.clone(), time);
                time += 1;

                if let Some(adjacencies) = self.get_adjacencies(node) {
                    for neighbor in adjacencies {
                        if !visited.contains(neighbor) {
                            // Tree edge
                            edge_types.insert((node.clone(), neighbor.clone()), EdgeType::Tree);
                            stack.push_back(neighbor);
                        } else if !departure_times.contains_key(neighbor) {
                            // Back edge
                            edge_types.insert((node.clone(), neighbor.clone()), EdgeType::Back);
                        } else if arrival_times.get(node) < arrival_times.get(neighbor) {
                            // Forward edge
                            edge_types.insert((node.clone(), neighbor.clone()), EdgeType::Forward);
                        } else {
                            // Cross edge
                            edge_types.insert((node.clone(), neighbor.clone()), EdgeType::Cross);
                        }
                    }
                }

                departure_times.insert(node.clone(), time);
                time += 1;
            }
        }

        edge_types
    }

    // Function to extract the DAG
    pub fn to_dag(&self) -> Self {
        let mut dag = AdjacencyGraph::new();

        // Perform DFS to classify edges
        let edge_types = self.dfs_classify_edges(self.nodes.iter().next().unwrap());

        for (from, to) in self.edges() {
            if let Some(edge_type) = edge_types.get(&(from.clone(), to.clone())) {
                if *edge_type == EdgeType::Tree || *edge_type == EdgeType::Forward {
                    dag.add_edge(from.clone(), to.clone()); // Keep only tree and forward edges
                }
            }
        }

        dag
    }

    // Util function for has_cycle that uses recursion
    pub fn has_cycle_util(
        &self,
        node: &V,
        visited: &mut HashMap<V, bool>,
        rec_stack: &mut HashMap<V, bool>,
    ) -> bool {
        // mark current node as visited and add to recursion stack
        visited.insert(node.clone(), true);
        rec_stack.insert(node.clone(), true);

        // recur for all neighbours
        // if any neighbour is visited and in rec_stack then cycle is detected
        if let Some(neighbours) = self.get_adjacencies(node) {
            for neighbour in neighbours {
                if !visited.contains_key(neighbour)
                    && self.has_cycle_util(neighbour, visited, rec_stack)
                {
                    return true;
                } else if rec_stack.contains_key(neighbour) {
                    return true;
                }
            }
        }

        // the node needs to be removed from the recursion stack before function ends
        rec_stack.remove(node);
        false
    }

    // Function that checks if the graph has a cycle
    pub fn has_cycle(&self) -> bool {
        // visited is a stack of bools, initiated as false, idem for rec_stack
        let mut visited = HashMap::new();
        let mut rec_stack = HashMap::new();

        for node in self.nodes.iter() {
            if !visited.contains_key(node) {
                if self.has_cycle_util(node, &mut visited, &mut rec_stack) {
                    return true;
                }
            }
        }

        false
    }

    pub fn remove_edge(&mut self, from: &V, to: &V) {
        if let Some(adjacencies) = self.adjacencies.get_mut(from) {
            adjacencies.remove(to);
        }
    }
}
