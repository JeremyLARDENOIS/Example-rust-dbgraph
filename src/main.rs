mod models;
use models::graph::Graph;
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

fn get_id() -> i32 {
    NEXT_ID.fetch_add(1, Ordering::Relaxed) as i32
}

fn main() {
    let mut g = Graph::new();
    /*
    drop() remove a vertex (including ingoing and outgoing relationships)
    */
    println!("* Add 2 vertices and 1 edge");
    g.addV();
    g.addV();
    g = g
        .addE("label")
        .from(g.V().id(0).unwrap())
        .to(g.V().id(1).unwrap()); // from vertex 0 to vertex 1 doesn't work

    println!("* Display vertices");
    println!("{}", g.V());

    println!("* Display edges");
    println!("{}", g.E());

    println!("* Add 1 vertex with property 'name' = 'Jeremy'");
    g = g.addV().property("name", "Jeremy");

    println!("* Display all vertices with property 'name' = 'Jeremy'");
    println!("{}", g.V().has("name", "Jeremy"));
    println!("* Add 1 edge between vertex 0 and vertex with property 'name' = 'Jeremy'");
    g = g
        .addE("label")
        .from(g.V().id(0).unwrap())
        .to(g.V().has("name", "Jeremy")._values[0].to_owned());

    println!("* Display all vertices with an edge out of vertex 0");
    println!("{}", g.V().id(0).unwrap().outV(g.clone()));

    println!("* Display all vertices with an edge into vertex 1");
    println!("{}", g.V().id(1).unwrap().inV(g.clone()));

    println!("* Display all edges out of vertex 0");
    println!("{}", g.V().id(0).unwrap().outE(g.clone()));

    println!("* Display all edges into vertex 1");
    println!("{}", g.V().id(1).unwrap().inE(g.clone()));

    println!("* Display graph");
    println!("{}", g);

    println!("* Drop edge with id 2");
    g.E().id(2).unwrap().drop(&mut g);

    println!("* Display graph");
    println!("{}", g);

    println!("* Drop vertex 0");
    g.V().id(0).unwrap().drop(&mut g);

    println!("* Display graph");
    println!("{}", g);

    println!("* Drop all vertices");
    // g = g.V().drop();
    g = g.drop();

    println!("* Display graph");
    println!("{}", g);
    println!(r"\o/")
}
