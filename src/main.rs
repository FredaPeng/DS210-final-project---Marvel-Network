use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use petgraph::visit::Bfs;
//use petgraph::prelude::*;
//use petgraph::algo::k_shortest_path::*;
use petgraph::graph::DiGraph;
use rustworkx_core::centrality::betweenness_centrality;
use petgraph::graph::NodeIndex;

// A struct to represent a hero in the Marvel universe
#[derive(Debug)]
struct Hero {
    name: String,
    comics: usize, // Add a field to store the number of comics the hero has appeared in
    centrality: f64,
}

impl Hero {
    // Create a new hero object with the given name and number of comics
    fn new(name: String, comics: usize) -> Hero {
        Hero {
            name,
            comics,
            centrality: 0.0,
        }
    }
}

// A struct to represent a graph of the Marvel universe
#[derive(Debug)]
struct MarvelGraph {
    graph: DiGraph<Hero, ()>, // Use the Hero struct as the node type
}

struct MostConnectedHero {
    name: String,
    centrality: f64,
}

impl MarvelGraph {
    // Build a graph representation of the Marvel universe from the provided edges data
    fn from_edges_data(data: Vec<Vec<String>>) -> MarvelGraph {
        let mut graph = MarvelGraph {
            graph: DiGraph::new(),
        };

        // Add a node for each hero in the data
        let mut hero_indices = HashMap::new();
        for edge in &data {
            let hero1 = edge[0].clone();
            let hero2 = edge[1].clone();

            // Create a new Hero object for each hero
            let hero1_obj = Hero::new(hero1.clone(), 1);
            let hero2_obj = Hero::new(hero2.clone(), 1);
            if !hero_indices.contains_key(&hero1) {
                hero_indices.insert(hero1.clone(), graph.graph.add_node(hero1_obj));
            }
            if !hero_indices.contains_key(&hero2) {
                hero_indices.insert(hero2.clone(), graph.graph.add_node(hero2_obj));
            }
    
            // Increment the number of comics each hero has appeared in
            let hero1_index = hero_indices[&hero1];
            let hero1_node = graph.graph.node_weight_mut(hero1_index).unwrap();
            hero1_node.comics += 1;
            let hero2_index = hero_indices[&hero2];
            let hero2_node = graph.graph.node_weight_mut(hero2_index).unwrap();
            hero2_node.comics += 1;
        }
    
        // Add an edge in the graph for each pair of heroes in the data
        for edge in data {
            let hero1 = edge[0].clone();
            let hero2 = edge[1].clone();
            graph.graph.add_edge(hero_indices[&hero1], hero_indices[&hero2], ());
        }
    
        graph
    }
    
    // Use degree centrality to find the hero who has appeared in the most comics with other heroes
    fn most_connected_hero(&self) -> MostConnectedHero {
        // Compute the betweenness centrality of each hero
        let centrality = betweenness_centrality(&self.graph, true, true, 100);
    
        // Find the hero with the highest centrality
        let mut most_connected_hero = MostConnectedHero {
            name: "".to_string(),
            centrality: 0.0,
        };
        for (node_index, score) in centrality.iter().enumerate() {
            let score: f64 = score.unwrap();
            if score > most_connected_hero.centrality {
                let hero = self.graph.node_weight(NodeIndex::new(node_index)).unwrap();
                most_connected_hero.name = hero.name.to_string();
                most_connected_hero.centrality = score;
            }
        }
        most_connected_hero
    }    
    
    // Compute the centrality of each hero in the graph
    fn compute_centrality(&self) -> HashMap<String, f64> {
        // Use the BetweennessCentrality struct to compute the betweenness centrality of each hero
        let centrality = betweenness_centrality(&self.graph, true, true, 100);
    
        // Create a HashMap to store the centrality of each hero
        let mut hero_centrality = HashMap::new();
        for score in <Vec<Option<f64>> as AsRef<Vec<Option<f64>>>>::as_ref(&centrality).iter().flatten() {
            hero_centrality.insert(score.to_string(), *score);
        }
        hero_centrality
    }
    
    // Use BFS to find the number of "degrees of separation" between two heroes in the Marvel network
    fn degrees_of_separation(&self, hero1: String, hero2: String) -> usize {
        let mut count = 0;
            // Get the indices of the two heroes in the graph
    let hero1_index = self.graph
        .node_indices()
        .find(|&i| self.graph.node_weight(i).unwrap().name == hero1)
        .unwrap();
    let hero2_index = self.graph
        .node_indices()
        .find(|&i| self.graph.node_weight(i).unwrap().name == hero2)
        .unwrap();

    // Use BFS to find the number of steps between the two heroes
    let mut bfs = Bfs::new(&self.graph, hero1_index);
    while let Some(node) = bfs.next(&self.graph) {
        count += 1;
        if node == hero2_index {
            break;
        }
    }
    count - 1
    }
}

fn main() {
    // Read the edges data from the "edges.csv" file
    let file = File::open("edges.csv").unwrap();
    let reader = BufReader::new(file);
    let mut edges_data = Vec::new();
    for line in reader.lines() {
    let line = line.unwrap();
    let heroes: Vec<&str> = line.split(",").collect();
    let hero1 = heroes[0].to_string();
    let hero2 = heroes[1].to_string();
    edges_data.push(vec![hero1, hero2]);
    }
    // Create a graph representation of the Marvel universe using the edges data
    let graph = MarvelGraph::from_edges_data(edges_data);

    // Use the compute_centrality method to compute the centrality of each hero
    let centrality = graph.compute_centrality();

    // Use the most_connected_hero method to find the hero who has appeared in the most comics
    let most_connected_hero = graph.most_connected_hero();

    // Use the degrees_of_separation method to find the number of degrees of separation between two heroes
    let hero1 = "SPIDER-MAN/PETER PARKER";
    let hero2 = "STACY, JILL";
    let degrees = graph.degrees_of_separation(hero1.to_string(), hero2.to_string());

    // Print the top 5 heroes in terms of percentage of comics they have appeared in
    let mut top_5: Vec<(&str, f64)> = centrality
        .iter()
        .map(|(hero, centrality)| {
            //let hero = hero.clone();
            let hero_name = hero.as_str();
            let hero_centrality = *centrality;
            (hero_name, hero_centrality)
        })
        .collect();
    top_5.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
    top_5.truncate(5);

    println!("Top 5 heroes in terms of percentage of comics they have appeared in:");
    for (hero, centrality) in top_5 {
        println!("{}: {:.2}%", hero, centrality * 100.0);
    }

    // Print the results
    println!("The hero who has the most connection is: {}", most_connected_hero.name);
    println!("The number of degrees of separation between {} and {} is: {}", hero1, hero2, degrees);
}
