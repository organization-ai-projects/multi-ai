/// Ce fichier définit des stratégies et primitives pour manipuler des données.
/// Rôle : Fournir des opérations primitives (`Primitive`) et des pipelines (`Strategy`) pour transformer des données ou évaluer leur efficacité.
use rand::{seq::SliceRandom, Rng};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Un pipeline (= stratégie candidate)
#[derive(Clone, Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
pub struct Strategy {
    pub pipeline: Vec<crate::primitive::Primitive>, // Utilisé uniquement via Experiment
    pub ancestry: Vec<String>,                      // Historique des mutations
}

impl Strategy {
    pub fn apply(&self, input: f64, custom_ops: &HashMap<String, Box<dyn Fn(f64) -> f64>>) -> f64 {
        self.pipeline
            .iter()
            .fold(input, |acc, prim| prim.apply(acc, custom_ops))
    }

    pub fn mutate(&self) -> Self {
        let mut rng = rand::rng();
        let mut new_pipeline = self.pipeline.clone();

        match rng.random_range(0..4) {
            0 if new_pipeline.len() > 1 => {
                new_pipeline.remove(rng.random_range(0..new_pipeline.len()));
            }
            1 => {
                new_pipeline.insert(
                    rng.random_range(0..=new_pipeline.len()),
                    crate::primitive::Primitive::random(),
                );
            }
            2 => {
                if let Some(p) = new_pipeline.choose_mut(&mut rng) {
                    *p = crate::primitive::Primitive::random();
                }
            }
            3 => {
                if rng.random_bool(0.2) {
                    new_pipeline.push(crate::primitive::Primitive::Custom(
                        "new_dynamic_op".to_string(),
                    ));
                }
            }
            _ => {}
        }
        let mut ancestry = self.ancestry.clone();
        ancestry.push(uuid::Uuid::new_v4().to_string());
        Strategy {
            pipeline: new_pipeline,
            ancestry,
        }
    }
}

// Stratégies de parsing
#[derive(Clone)]
pub enum ParserStrategy {
    BasicHtml,
    Wiktionary,
}

impl ParserStrategy {
    pub async fn scrape_and_parse(&self, url: &str) -> Result<Vec<String>, ()> {
        let html = reqwest::get(url)
            .await
            .map_err(|_| ())?
            .text()
            .await
            .map_err(|_| ())?;
        match self {
            ParserStrategy::BasicHtml => parse_html_basic(&html),
            ParserStrategy::Wiktionary => parse_html_wiktionary(&html),
        }
    }
}

pub fn parse_html_basic(html: &str) -> Result<Vec<String>, ()> {
    let doc = Html::parse_document(html);
    let selector = Selector::parse("a").unwrap();
    let words = doc
        .select(&selector)
        .filter_map(|e| e.text().next().map(|s| s.trim().to_string()))
        .filter(|s| !s.is_empty() && s.len() < 30)
        .collect::<Vec<_>>();
    Ok(words)
}

pub fn parse_html_wiktionary(html: &str) -> Result<Vec<String>, ()> {
    let doc = Html::parse_document(html);
    let selector = Selector::parse("span.mot").unwrap_or_else(|_| Selector::parse("a").unwrap());
    let words = doc
        .select(&selector)
        .filter_map(|e| e.text().next().map(|s| s.trim().to_string()))
        .filter(|s| !s.is_empty() && s.len() < 30)
        .collect::<Vec<_>>();
    Ok(words)
}
