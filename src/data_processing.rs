use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;  
use std::collections::HashSet;  
use crate::graph::Graph;

// hero's appearance in a comic
#[derive(Debug, Clone)]
pub struct CharacterAppearance {
    pub hero: String,
    pub comic: String,
}

impl CharacterAppearance {
    // new line for data file 
    pub fn new_from_line(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 2 {
            return None;
        }
        let hero = parts[0].to_string();
        let comic = parts[1].trim().to_string(); 
        Some(CharacterAppearance { hero, comic })
    }

    // make all names uppercase
    pub fn to_uppercase(&mut self) {
        self.hero = self.hero.to_uppercase();
        self.comic = self.comic.to_uppercase();
    }
}

// parse the file into a vector
pub fn read_file(file_path: &Path) -> io::Result<Vec<CharacterAppearance>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut appearances = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        if let Some(mut appearance) = CharacterAppearance::new_from_line(&line) {
            appearance.to_uppercase();
            appearances.push(appearance);
        }
    }

    Ok(appearances)
}

//tests
#[cfg(test)]
mod tests {
    use super::*;
    //testing uppercase
    #[test]
    fn test_ensure_uppercase_connections() {
        let mut connections = vec![
            CharacterConnection { character_a: "iron man".to_string(), character_b: "thor".to_string() },
            CharacterConnection { character_a: "Black Widow".to_string(), character_b: "HAWKEYE".to_string() },
        ];

        ensure_uppercase_connections(&mut connections);

        for connection in connections.iter() {
            assert_eq!(connection.character_a, connection.character_a.to_uppercase());
            assert_eq!(connection.character_b, connection.character_b.to_uppercase());
        }
    }

}

    // graph from the list of character appearances
pub fn build_graph(appearances: &[CharacterAppearance]) -> Graph {
    let mut graph = Graph::new();
    let mut comics = HashMap::new();

    // group the heroes by the comic
    for appearance in appearances {
        comics.entry(&appearance.comic)
              .or_insert_with(HashSet::new)
              .insert(&appearance.hero);
    }

    // connect the heroes in the same comics
    for heroes in comics.values() {
        let hero_list: Vec<&String> = heroes.iter().map(|&hero| hero).collect();
        for (i, &hero1) in hero_list.iter().enumerate() {
            for &hero2 in hero_list.iter().skip(i + 1) {
                graph.add_edge(hero1, hero2);
            }
        }
    }

    graph
}

