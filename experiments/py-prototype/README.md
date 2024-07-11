# Code base in python for prototyping

This is just a simple code base in python for prototyping. It is intended to be used as a starting point to test ideas and then switch to a more robust implementation in Rust

## Data retrival

Use the graphs from the GFA to retrive the data.

- Make the graphs DAGs with the already implemented algorithm in rust
- Create some sort of file where for each node we save the strings associated and the edges (directed)
- Given $\Omega$ the alphabets of the strings associated with the nodes, create $|\Omega|$ different DAGs, one for each alphabet. From this point each node will have associated just an integer, representing how many times that character appears in the string that was associated with the node, and the directed edges.

### Preprocessing

In this step we need to add to each node new metadata. This metadata will be a set of integers, called `out`. It will be build recursively. The `out` set of a node $n$ will be the union of the `out` sets of the children where each element is incremented by the node weight.

```python
for node in nx.topological_sort(graph):
    node.out = set()
    for child in node.children:
        node.out = node.out.union(child.out)

for x in node.out:
    x += node.weight
```

## Father recursive approach

In this approach we will need a node class with the following attributes:

- `weight (int)` : the weight of the node
- `successor_id (int)` : the id of the optimal successor
- `offset (list)` : the position (0-indexed) of the elements in the `out` set of the father node that are in the `out` set of the node

**ALGORITHM IDEA:** We want to store every node that has a father implicitly and explicitly just the nodes with out degree zero. We visit the nodes in topological ordering:

- If the node has out degree zero, we do nothing to its out set
- If a node has out degree greater than zero, we need to find the optimal successor.
- We store the `successor_id`
- Compute the `offset` list (has to be done efficiently)

#### Optimal successor

We have two options

- The optimal successor is node in the successor list with the shortest out set: this will result in a more compressed offset list:
- The successor withing the shortest path from the node to a node with out degree zero: this will result in a more efficient query but with more space usage: _this implies that instead of a successor id we will need a list of successor ids_

#### Query

If we want to query the value of the out set associated to a node, we will need to access, for each element in the `offset` list, the value of the `out` set of the father node with that index. We do this recursively until we reach a node with out degree zero.

This is far from trivial: I will basically create a path that goes from the queried node to an explicit node. During this path we need to save an additional value that is the sum of the weights of the nodes in the path (without the starting one). Once we arrive to an explicit node with a certain index, we can access that element in its out set subtract to it the sum up to that point and return the result.

### Compression of the offset list

The offset list is just a list of increasing integers, so we can use Elias-Fano. We need to do the following step to optimize the compression:

- We need to store the first element of the list explicitly
- Shift the list by minus the first element
- Subtract the rank (starting from 0) to each element (where the rank it the position of the element in the list)
- Then apply Elias-Fano

On the top of the compressed bitmap that will return elias-fano, we need to store an RRR data structure in order to do rank, select and access in `O(1)`. This will allow us to access the elements in the offset list in `O(1)` time.

## Topological generations approach

We devide the DAG in topological levels. For a node in a level, all its successors are in the following level and all its predecessors are in the previous level.

**ALGORITHMIC IDEA**: There will be levels where the nodes have not associated the set `out`. If we want to query those nodes and retrive the set `out` we will need to go down to the previous level (explict), do a set union with all the set `out` of the predecessors and then sum to each element the weight of the node.

- **Easy option:** The levels are explicit one after the other. This means that whenever a query is done on a node that is explicit, we can just return the associated `out` set. If the queried node is in an implicit level, we know for sure that all its predecessors are in the previous level (that is explict) and we can just do the union of the `out` sets of the predecessors and sum the weight of the node to each element.

- **Hard option:** We store levels explicitly less frequently, for example one every three etc... In this way, as before, if the queried node is in an explicit level, we can just return the associated `out` set. If the queried node is in an implicit level, we need to go down recursively to the previous levels until we found an explicit one. The problem is that to do a query of a node, I need the values of the set out of its predecessors, that if they are implicit too, we will risk to do an exponential number of queries.

This means that we need to store a list of list that contains the nodes in each level. And also a list of bools that tells us if the level is explicit or not.

Each sub-list will contain the ids of the nodes in that level. For each node we need to store

- `weight (int)` : the weight of the node
- `predecessors (list)` : the ids of the predecessors
- `out (Option(set))` : the set of integers associated with the node, if the node is in an explicit level, nothing if it is in an implicit level

### Compression

Both the ids in the levels and the values in the out sets are increasing integers, so we can use Elias-Fano. We can use the same steps discussed before to compress the offset list.

## Extra

- I will need metrics about time efficiency and space occupance in order to compare the two methods and choose the best one.
- I will need something very general to compress the list of integers with Elias Fano and build above it the RRR data structure.
