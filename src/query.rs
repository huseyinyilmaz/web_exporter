use crate::settings;
use crate::results;
use crate::errors;
use std::error;
use reqwest;
use scraper::Html;
use scraper::Selector;
use futures::future::{join_all};
// use tokio::time::{delay_for, Duration};

async fn run_query(q: &settings::Query) -> Result<results::QueryResult, Box<dyn error::Error>> {
    let response = reqwest::get(&q.url).await?;
    let status = response.status().as_u16();
    let body = response.text().await?;
    let document = Html::parse_document(&body);
    // TODO remove unwrap
    match Selector::parse(&q.query) {
        Ok(selector) => {
          let results = document.select(&selector);
          let count = results.fold(0, |acc, _| acc + 1);
          let query_result = results::QueryResult{
              completed: true,
              url: q.url.clone(),
              query: q.query.clone(),
              count: count,
              status: status,
              error: String::from("")};
          Ok(query_result)
        },
        Err(err) => Err(Box::new(errors::QueryError{description: format!("{:?}", err)})),
    }
}

async fn query(q: &settings::Query) -> results::QueryResult {
    match run_query(q).await {
        Ok(r) => r,
        Err(err) => results::QueryResult{
            completed: false,
            url: q.url.clone(),
            query: q.query.clone(),
            count: 0,
            status: 0,
            error: err.to_string()}
    }
}


pub async fn query_targets(s: &settings::Settings) -> results::Result {
    info!("Starting crawling targets.");
    let query_results: Vec<results::QueryResult> = join_all(
        s.queries
        .iter()
        .map(query)
    ).await;
    info!("crawling target is complete.");
    let result = results::Result {
        query_results: query_results,
    };
    info!("{:?}", result);
    return result;

}
