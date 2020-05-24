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
    size: usize,
}
// use tokio::time::{delay_for, Duration};
async fn query_url(url: &str) -> Result<QueryResponse, Box<dyn error::Error>> {
    let response = reqwest::get(url).await?;
    let status = response.status().as_u16();
    let body = response.text().await?;
    let size = body.len();
    let document = Html::parse_document(&body);
    Ok(QueryResponse {
        status,
        document,
        size,
    })
}

// async fn process_target(target: &settings::Target) -> Vec<results::QueryResult> {
async fn process_target(target: &settings::Target) -> results::TargetResult {
    let start = time::Instant::now();
    let raw_response = &query_url(&target.url).await;
    let duration = start.elapsed().as_millis();
    let mut query_results = Vec::new();
    match &raw_response {
        // There was an error with the request.
        Err(err) => {
            warn!("Network Error: url: {:?}, error: {:?}", target.url, err);
            results::TargetResult {
                url: target.url.clone(),
                status: 0,
                error: true,
                size: 0,
                duration,
                // No need to go through queries response
                query_results,
            }
        }
        Ok(response) => {
            for q in &target.queries {
                query_results.push(match &Selector::parse(&q) {
                    Err(err) => {
                        warn!("Query Parse Error: url: {:?}, error: {:?}", target.url, err);
                        // None means we could not parse that query.
                        results::QueryResult {
                            query: q.clone(),
                            count: None,
                        }
                    }
                    Ok(selector) => {
                        let results = response.document.select(&selector);
                        let count = results.fold(0, |acc, _| acc + 1);
                        results::QueryResult {
                            query: q.clone(),
                            count: Some(count),
                        }
                    }
                });
            }
            results::TargetResult {
                url: target.url.clone(),
                status: response.status,
                error: false,
                size: response.size,
                duration,
                // No need to go through queries response
                query_results,
            }
        }
    }
}

pub async fn process_targets(s: &settings::Settings) -> results::Result {
    info!("Starting crawling targets.");
    let target_results: Vec<results::TargetResult> =
        join_all(s.targets.iter().map(process_target)).await;
    // .into_iter()
    // .flatten()
    // .collect();
    info!("crawling target is complete.");
    let result = results::Result { target_results };
    debug!("{:?}", result);
    result
}
