use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
};

#[derive(Debug)]
pub struct AdjacencyGraph<V>
where
    V: Hash + Eq + Clone,
{
    nodes: HashSet<V>,
    adjacencies: HashMap<V, HashSet<V>>,
}

#[allow(dead_code)]
impl<V> AdjacencyGraph<V>
where
    V: Hash + Eq + Clone + Debug,
{
    pub fn new() -> Self {
        AdjacencyGraph {
            nodes: HashSet::new(),
            adjacencies: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: V) {
        // O(1)
        self.nodes.insert(node);
    }

    pub fn add_edge(&mut self, from: V, to: V) {
        self.add_node(from.clone());
        self.add_node(to.clone());

        // O(1)
        // Adding the edge to the adjacency list
        self.adjacencies
            .entry(from)
            .or_insert_with(HashSet::new)
            .insert(to);
    }

    pub fn get_adjacencies(&self, node: &V) -> Option<&HashSet<V>> {
        self.adjacencies.get(node)
    }

    pub fn adjacencies(&self) -> &HashMap<V, HashSet<V>> {
        &self.adjacencies
    }

    pub fn edges(&self) -> impl Iterator<Item = (&V, &V)> + '_ {
        // &'_ is a lifetime specifier
        self.adjacencies
            .iter()
            .flat_map(|(from, tos)| tos.iter().map(move |to| (from, to))) // O(|E|)
    }

    // Function that inverts all the edges in the graph
    pub fn opposite(&self) -> AdjacencyGraph<&V> {
        let mut opposite = AdjacencyGraph::new();

        // O(|E|) where E is the set of edges
        for (from, to) in self.edges() {
            opposite.add_edge(to, from);
        }

        opposite
    }

    pub fn has_edge(&self, from: &V, to: &V) -> bool {
        if let Some(adjacencies) = self.get_adjacencies(from) {
            adjacencies.contains(&to.to_owned())
        } else {
            false
        }
    }

    pub fn dfs<'a>(&'a self, node: &'a V) -> impl Iterator<Item = V> + 'a {
        let mut visited = HashSet::new();
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

    pub fn remove_self_loops(&mut self) {
        let mut to_remove = Vec::new();

        for (from, tos) in self.adjacencies.iter_mut() {
            if tos.contains(from) {
                to_remove.push((from.clone(), from.clone()));
            }
        }

        for (from, to) in to_remove {
            self.adjacencies.get_mut(&from).unwrap().remove(&to);
        }
    }

    // takes a node, visited and rec_stack as arguments
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
        for neighbour in self.get_adjacencies(node).unwrap() {
            if !visited.contains_key(neighbour)
                && self.has_cycle_util(neighbour, visited, rec_stack)
            {
                return true;
            } else if rec_stack.contains_key(neighbour) {
                return true;
            }
        }

        // the node needs to be removed from the recursion stack before function ends
        rec_stack.remove(node);
        false
    }

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
