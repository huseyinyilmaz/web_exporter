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
async fn query_target(target: &settings::Target) -> Result<QueryResponse, Box<dyn error::Error>> {
    let client = reqwest::Client::new();
    let url = &target.url;
    let mut req = match target
        .method

        .as_ref()
        .unwrap_or(&settings::TargetMethod::GET)
    {
        settings::TargetMethod::GET => client.get(url),
        settings::TargetMethod::POST => client.post(url),
    };
    if let Some(body_string) = &target.body {
        req = req.body(body_string.to_string());
    } else if let Some(formdata) = &target.formdata {
        let mut form = reqwest::multipart::Form::new();
        for (key, value) in formdata {
            form = form.text(key.clone(), value.clone());
        }
        req = req.multipart(form);
    }
    if let Some(queryparameters) = &target.queryparameters {
        // let multipart_form = reqwest::multipart::Form::new();
        // for (key, value) in queryparameters {
        //     form = form.text(key, value);
        // }
        req = req.query(queryparameters);
    }

    if let Some(headers) = &target.headers {
        let mut headers_map = reqwest::header::HeaderMap::new();
        for (key, value) in headers {
            headers_map.insert(
                reqwest::header::HeaderName::from_bytes(key.as_bytes())?,
                reqwest::header::HeaderValue::from_str(value)?,
            );
        //     form = form.text(key, value);
         }
        req = req.headers(headers_map);
    }



    let response = req.send().await?;
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
async fn process_target(target: &settings::Target) -> results::TargetResult<'_> {
    let start = time::Instant::now();
    let raw_response = &query_target(&target).await;
    let duration = start.elapsed().as_millis();
    let mut query_results = Vec::new();
    let default_queries = &Vec::new();
    match &raw_response {
        // There was an error with the request.
        Err(err) => {
            warn!("Network Error: url: {:?}, error: {:?}", target.url, err);
            results::TargetResult {
                url: target.url.clone(),
                status: 0,
                method: &target
                    .method
                    .as_ref()
                    .unwrap_or(&settings::TargetMethod::GET), // TODO do not repeat default value.
                error: true,
                size: 0,
                duration,
                // No need to go through queries response
                query_results,
            }
        }
        Ok(response) => {
            for q in target.queries.as_ref().unwrap_or(default_queries) {
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
                method: target
                    .method
                    .as_ref()
                    .unwrap_or(&settings::TargetMethod::GET),
                error: false,
                size: response.size,
                duration,
                // No need to go through queries response
                query_results,
            }
        }
    }
}

pub async fn process_targets<'result, 'settings: 'result>(
    s: &'settings settings::Settings,
) -> results::Result<'result> {
    info!("Starting crawling targets.");
    // empty vector will be assigned from default value.
    let targets = s.targets.as_ref().unwrap();
    let target_results: Vec<results::TargetResult> =
        join_all(targets.iter().map(process_target)).await;
    // .into_iter()
    // .flatten()
    // .collect();
    info!("crawling target is complete.");
    let result = results::Result { target_results };
    debug!("{:?}", result);
    result
}
