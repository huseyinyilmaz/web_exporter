use crate::settings;
use crate::results;
use std::error;
use reqwest;
use scraper::Html;
use scraper::Selector;
use futures::future::{join_all};
use tokio::time::{delay_for, Duration};

async fn run_count_query(q: &settings::CountQuery) -> Result<u32, Box<dyn error::Error>> {
    let body = reqwest::get(&q.url)
        .await?
        .text()
        .await?;
    let document = Html::parse_document(&body);
    // TODO remove unwrap
    let selector = Selector::parse(&q.query).unwrap();
    let results = document.select(&selector);
    let result_length = results.fold(0, |acc, _| acc + 1);
    Ok(result_length)
}

async fn count_query(q: &settings::CountQuery) -> results::CountResult {
    match run_count_query(q).await {
        Ok(count) => results::CountResult{url: q.url.clone(), query: q.query.clone(), count: count, error: String::from("")},
        Err(err) => results::CountResult{url: q.url.clone(), query: q.query.clone(), count: 0, error: err.to_string()}
    }
}

pub async fn crawl_targets(s: &settings::Settings) -> Result<(), Box<dyn error::Error>> {
    info!("Starting crawling targets.");
    let count_results: Vec<results::CountResult> = join_all(
        s.count_queries
        .iter()
        .map(count_query)
    ).await;
    info!("{:?}", count_results);
    info!("crawling target is complete.");
    Ok(())
}

pub async fn run_crons(s: &settings::Settings) -> Result<(), Box<dyn error::Error>> {
    loop {
        crawl_targets(s).await?;
        delay_for(Duration::from_secs(s.interval_seconds)).await;
    };
}
