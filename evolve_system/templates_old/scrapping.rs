use reqwest;
use scraper::{Html, Selector};

pub async fn scrap_larousse(word: &str) -> Option<String> {
    let url = format!("https://www.larousse.fr/dictionnaires/francais/{}", word);
    let body = reqwest::get(&url).await.ok()?.text().await.ok()?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("li.DivisionDefinition").unwrap();
    document
        .select(&selector)
        .next()
        .map(|def| def.text().collect::<Vec<_>>().join(" "))
}
