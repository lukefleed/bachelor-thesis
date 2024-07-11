# Rust tool to work with GFA files

At the moment we are just interest in doing the following steps with a GFA file:

1. Check if its a DAG (it's not)
2. Convert it to a DAG by removing the edges that create cycles (look at function `to_dag`)
3. Write it in a new file

In the following folder we can run the following command

```bash
cargo run -r -- convert -i path/to/input.gfa -o path/to/output.tsv
```

where we of course will change `path/to/input.gfa` and `path/to/output.tsv` to the correct paths.

## The generated .tsv file

The generated file will have the following format:

- The first line will have the number of nodes in the graph, let's call it `n`
- The following `n` lines will have two tab separated values, the first one is the node id and the second one is the node sequence
- The remaining line will have two tab separated values, the first one is the source node id and the second one is the target node id (i.e we are giving a list of edges)
