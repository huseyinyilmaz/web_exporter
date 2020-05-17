mod settings;
mod results;
mod errors;
mod query;
use warp::Filter;
use std::convert::Infallible;
// use std::error;
use std::sync::Arc;
use pretty_env_logger;
use std::time;

#[macro_use]
extern crate log;


async fn get_metrics(s: Arc<settings::Settings>) -> Result<impl warp::Reply, Infallible> {
    let start = time::Instant::now();
    let result = query::process_targets(&s).await;
    let duration = start.elapsed();
    Ok(vec![
        result.to_string(),
        format!("web_exporter_scrape_duration_milliseconds {}", duration.as_millis()),
    ].join("\n"))
}

pub async fn run() {
    pretty_env_logger::init();
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    // XXX we are unwrapping a result here. handle errors better.
    match settings::Settings::new() {
        Ok(setting) => {
            let s = Arc::new(setting);
            info!("settings: {:?}", s.clone());
            let state = warp::any().map(move || s.clone());

            let metrics = warp::path("metrics").and(state).and_then(get_metrics);

            let routes = metrics;
            let server = warp::serve(routes).run(([0, 0, 0, 0], 3030));

            server.await;
            info!("Initialization Complete!");
        },
        Err(err) => error!("Cannot parse configuration file: {:?}", err),
    }
}
