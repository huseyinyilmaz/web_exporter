use crate::results;
use crate::settings;
// use crate::errors;
use futures::future::join_all;
// use reqwest;
use scraper::Html;
use scraper::Selector;
use std::error;
// use tokio::stream::StreamExt;
// use futures::prelude::*;
// use futures::stream;
// use futures::stream::StreamExt;
use std::time;

struct QueryResponse {
    status: u16,
    document: Html,
    duration: u128,
    size: usize,
}
// use tokio::time::{delay_for, Duration};
async fn query_url(url: &str) -> Result<QueryResponse, Box<dyn error::Error>> {
    let start = time::Instant::now();
    let response = reqwest::get(url).await?;
    let status = response.status().as_u16();
    let body = response.text().await?;
    let duration = start.elapsed().as_millis();
    let size = body.len();
    let document = Html::parse_document(&body);
    Ok(QueryResponse {
        status,
        document,
        duration,
        size,
    })
}

async fn process_target(target: &settings::Target) -> Vec<results::QueryResult> {
    let response = &query_url(&target.url).await;
    let mut result = Vec::new();
    for q in &target.queries {
        result.push(match (&response, &Selector::parse(&q)) {
            (Ok(response), Ok(selector)) => {
                let results = response.document.select(&selector);
                let count = results.fold(0, |acc, _| acc + 1);
                results::QueryResult {
                    count,
                    error: false,
                    url: target.url.clone(),
                    duration: response.duration,
                    query: q.clone(),
                    status: response.status,
                    size: response.size,
                }
            }
            (Err(err), _) => {
                warn!("Network Error: url: {:?}, error: {:?}", target.url, err);
                results::QueryResult {
                    error: true,
                    duration: 0,
                    url: target.url.clone(),
                    query: q.clone(),
                    count: 0,
                    status: 0,
                    size: 0,
                }
            }
            (_, Err(err)) => {
                warn!("Query Parse Error: url: {:?}, error: {:?}", target.url, err);
                results::QueryResult {
                    error: true,
                    duration: 0,
                    url: target.url.clone(),
                    query: q.clone(),
                    count: 0,
                    status: 0,
                    size: 0,
                }
            }
        });
    } // for loop
    result
}

pub async fn process_targets(s: &settings::Settings) -> results::Result {
    info!("Starting crawling targets.");
    let query_results: Vec<results::QueryResult> = join_all(s.targets.iter().map(process_target))
        .await
        .into_iter()
        .flatten()
        .collect();
    info!("crawling target is complete.");
    let result = results::Result { query_results };
    debug!("{:?}", result);
    result
}
