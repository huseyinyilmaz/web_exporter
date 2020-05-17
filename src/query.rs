use crate::settings;
use crate::results;
// use crate::errors;
use std::error;
use reqwest;
use scraper::Html;
use scraper::Selector;
use futures::future::{join_all};
// use tokio::stream::StreamExt;
// use futures::prelude::*;
// use futures::stream;
// use futures::stream::StreamExt;
struct QueryResponse {
    status: u16,
    document: Html,
}
// use tokio::time::{delay_for, Duration};
async fn query_url(url: &String) -> Result<QueryResponse, Box<dyn error::Error>> {
    let response = reqwest::get(url).await?;
    let status = response.status().as_u16();
    let body = response.text().await?;
    let document = Html::parse_document(&body);
    Ok(QueryResponse {
        status: status,
        document: document,
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
                results::QueryResult{
                    error: false,
                    url: target.url.clone(),
                    query: q.clone(),
                    count: count,
                    status: response.status,
                }
            },
            (Err(err), _) => {
                warn!("Network Error: url: {:?}, error: {:?}", target.url, err);
                results::QueryResult{
                    error: true,
                    url: target.url.clone(),
                    query: q.clone(),
                    count: 0,
                    status: 0,
                }
            },
            (_, Err(err)) => {
                warn!("Query Parse Error: url: {:?}, error: {:?}", target.url, err);
                results::QueryResult{
                    error: true,
                    url: target.url.clone(),
                    query: q.clone(),
                    count: 0,
                    status: 0,
                }
            },
        });
    } // for loop
    return result;
}

pub async fn process_targets(s: &settings::Settings) -> results::Result {
    info!("Starting crawling targets.");
    let query_results: Vec<results::QueryResult> = join_all(
        s.targets
            .iter()
            .map(process_target)

    ).await.into_iter().flatten().collect();
    info!("crawling target is complete.");
    let result = results::Result {
        query_results: query_results,
    };
    debug!("{:?}", result);
    return result;

}
