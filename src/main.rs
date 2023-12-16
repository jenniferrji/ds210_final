mod data_processing;
mod graph;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("/Users/jennifer/Desktop/DS210/ds210_final_marvel/data/edges.txt");

    // build graph
    let appearances = data_processing::read_file(&path)?;
    let graph = data_processing::build_graph(&appearances);

    // analyze graph
    let degree_distribution = graph.degree_distribution();
    let min_connections = graph.min_connections();
    let max_connections = graph.max_connections();

    // neighbors at distance 2 
    let neighbors_at_distance_two = graph.neighbors_at_distance_two();
    for (hero, neighbors) in &neighbors_at_distance_two {
        println!("Hero: {}, Neighbors at distance 2: {}", hero, neighbors.len());
    }

    // print
    println!("Degree Distribution Summary:");
    for (&degree, &count) in &degree_distribution {
        println!("Characters with {} connections: {}", degree, count);
    }
    println!("Minimum number of connections for a hero: {}", min_connections);
    println!("Maximum number of connections for a hero: {}", max_connections);

    Ok(())
}
