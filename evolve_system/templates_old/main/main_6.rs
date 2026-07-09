mod knowledge;
mod strategies;

use knowledge::KnowledgeGraph;
use strategies::{ParserStrategy, parse_html_basic};
use rand::seq::SliceRandom;
use ron::de::from_str;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Serialize, Deserialize)]
pub struct ExplorerState {
    pub queue: VecDeque<String>,
    pub graph: KnowledgeGraph,
    pub tried_urls: Vec<String>,
}

impl ExplorerState {
    pub fn save(&self, path: &str) {
        let ron = ron::to_string(self).unwrap();
        fs::write(path, ron).unwrap();
    }
    pub fn load(path: &str) -> Self {
        let content = fs::read_to_string(path).unwrap();
        ron::from_str(&content).unwrap()
    }
}

#[tokio::main]
async fn main() {
    // 1. Charger seeds + restore queue
    let seeds: Vec<String> = ron::from_str(&fs::read_to_string("seeds.ron").unwrap()).unwrap();
    let mut queue: VecDeque<String> = seeds.into();
    let mut graph = KnowledgeGraph::new();
    let mut tried_urls = Vec::new();

    // Restore
    if let Ok(state) = fs::read_to_string("explorer_state.ron") {
        let s: ExplorerState = ron::from_str(&state).unwrap();
        queue = s.queue;
        graph = s.graph;
        tried_urls = s.tried_urls;
    }

    // 2. Liste de parseurs/stratégies évolutives
    let mut strategies = vec![ParserStrategy::BasicHtml, ParserStrategy::Wiktionary];

    loop {
        // a. Prendre url suivante
        if let Some(url) = queue.pop_front() {
            println!("Exploration de {url}");
            tried_urls.push(url.clone());

            // b. Essayer tous les parseurs en random order
            strategies.shuffle(&mut rand::thread_rng());
            let mut found = false;
            for strat in &strategies {
                match strat.scrape_and_parse(&url).await {
                    Ok(words) if !words.is_empty() => {
                        println!("Ajout au graph: {:?}", words);
                        graph.add_words(&url, &words);
                        // Découverte : nouveaux liens à explorer !
                        for w in words {
                            if !tried_urls.contains(&w) {
                                // On génère un nouvel URL à partir du mot
                                let new_url = format!("https://fr.wiktionary.org/wiki/{}", w);
                                queue.push_back(new_url);
                            }
                        }
                        found = true;
                        break;
                    }
                    _ => continue,
                }
            }
            if !found {
                println!("Échec pour {url}");
            }

            // c. Persistance régulière (checkpoint)
            let state = ExplorerState { queue: queue.clone(), graph: graph.clone(), tried_urls: tried_urls.clone() };
            state.save("explorer_state.ron");
        } else {
            println!("Queue vide : rien à explorer, dodo 10s");
            sleep(Duration::from_secs(10)).await;
        }
    }
}
