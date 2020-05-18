mod query;
mod results;
mod settings;
use std::convert::Infallible;
use warp::Filter;
use std::sync::Arc;
use std::time;

#[macro_use]
extern crate log;

async fn get_metrics(s: Arc<settings::Settings>) -> Result<impl warp::Reply, Infallible> {
    let start = time::Instant::now();
    let result = query::process_targets(&s).await;
    let duration = start.elapsed();
    Ok(vec![
        result.to_string(),
        format!(
            "web_exporter_scrape_duration_milliseconds {}",
            duration.as_millis()
        ),
    ]
    .join("\n"))
}

pub async fn run() {
    pretty_env_logger::init_custom_env("WEB_EXPORTER_LOG_LEVEL");
    match settings::Settings::new() {
        Ok(setting) => {
            let addr = setting.ip_address.clone();
            let port = setting.port.clone();
            let path = setting.metrics_path.clone();
            let s = Arc::new(setting);

            info!("settings: {:?}", s.clone());
            let state = warp::any().map(move || s.clone());
            let metrics = warp::path(path).and(state).and_then(get_metrics);
            let routes = metrics;
            let server = warp::serve(routes).run((addr, port));
            server.await;
            info!("Initialization Complete!");
        }
        Err(err) => error!("Cannot parse configuration file: {:?}", err),
    }
}
