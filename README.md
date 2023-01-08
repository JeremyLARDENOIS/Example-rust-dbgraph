# Example-rust-dbgraph

This is a simple example of how to implement a graph database in Rust.

## Usage

```bash
cargo run
```

## Output

```bash
* Add 2 vertices and 1 edge
* Display vertices
v[0]
v[1]
* Display edges
e[2][0-label>1]
* Add 1 vertex with property 'name' = 'Jeremy'
* Display all vertices with property 'name' = 'Jeremy'
v[3]
* Add 1 edge between vertex 0 and vertex with property 'name' = 'Jeremy'
* Display all vertices with an edge out of vertex 0
v[1]
v[3]
* Display all vertices with an edge into vertex 1
v[0]
* Display all edges out of vertex 0
e[2][0-label>1]
e[4][0-label>3]
* Display all edges into vertex 1
e[2][0-label>1]
* Display graph
Graph { vertices: 3, edges: 2 }
* Drop edge with id 2
* Display graph
Graph { vertices: 3, edges: 1 }
* Drop vertex 0
* Display graph
Graph { vertices: 2, edges: 0 }
* Drop all vertices
* Display graph
Graph { vertices: 0, edges: 0 }
\o/
```