use reqwest;
use scraper::{Html, Selector};

#[derive(Clone)]
pub enum ParserStrategy {
    BasicHtml,
    Wiktionary,
    // ...Ajoute-en d'autres/mutables plus tard
}

impl ParserStrategy {
    pub async fn scrape_and_parse(&self, url: &str) -> Result<Vec<String>, ()> {
        let html = reqwest::get(url).await.map_err(|_| ())?.text().await.map_err(|_| ())?;
        match self {
            ParserStrategy::BasicHtml => parse_html_basic(&html),
            ParserStrategy::Wiktionary => parse_html_wiktionary(&html),
        }
    }
}

pub fn parse_html_basic(html: &str) -> Result<Vec<String>, ()> {
    let doc = Html::parse_document(html);
    let selector = Selector::parse("a").unwrap();
    let words = doc.select(&selector)
        .filter_map(|e| e.text().next().map(|s| s.trim().to_string()))
        .filter(|s| !s.is_empty() && s.len() < 30)
        .collect::<Vec<_>>();
    Ok(words)
}

pub fn parse_html_wiktionary(html: &str) -> Result<Vec<String>, ()> {
    let doc = Html::parse_document(html);
    let selector = Selector::parse("span.mot").unwrap_or_else(|_| Selector::parse("a").unwrap());
    let words = doc.select(&selector)
        .filter_map(|e| e.text().next().map(|s| s.trim().to_string()))
        .filter(|s| !s.is_empty() && s.len() < 30)
        .collect::<Vec<_>>();
    Ok(words)
}
