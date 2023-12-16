use std::collections::{HashMap, HashSet};

pub struct Graph {
    adj_list: HashMap<String, HashSet<String>>,
}

impl Graph {
    // make a new graph 
    pub fn new() -> Self {
        Graph { adj_list: HashMap::new() }
    }

    //undirected edge between two heroes
    pub fn add_edge(&mut self, hero1: &str, hero2: &str) {
        self.adj_list.entry(hero1.to_string())
                     .or_insert_with(HashSet::new)
                     .insert(hero2.to_string());
        self.adj_list.entry(hero2.to_string())
                     .or_insert_with(HashSet::new)
                     .insert(hero1.to_string());
    }

    // calculate the degree distrbibution 
    pub fn degree_distribution(&self) -> HashMap<usize, usize> {
        let mut distribution = HashMap::new();
        for connections in self.adj_list.values() {
            *distribution.entry(connections.len()).or_insert(0) += 1;
        }
        distribution
    }

    // min ## of connections a hero has
    pub fn min_connections(&self) -> usize {
        self.adj_list.values().map(|c| c.len()).min().unwrap_or(0)
    }

    // max ## of connections a hero has.
    pub fn max_connections(&self) -> usize {
        self.adj_list.values().map(|c| c.len()).max().unwrap_or(0)
    }
    //neighbors
    pub fn neighbors_at_distance_two(&self) -> HashMap<String, HashSet<String>> {
        let mut neighbors_dist_two = HashMap::new();

        for (node, neighbors) in &self.adj_list {
            let mut dist_two_neighbors = HashSet::new();
            for neighbor in neighbors {
                if let Some(second_neighbors) = self.adj_list.get(neighbor) {
                    for second_neighbor in second_neighbors {
                        if second_neighbor != node && !neighbors.contains(second_neighbor) {
                            dist_two_neighbors.insert(second_neighbor.clone());
                        }
                    }
                }
            }
            neighbors_dist_two.insert(node.clone(), dist_two_neighbors);
        }

        neighbors_dist_two
    }
}
