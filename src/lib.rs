// mod cron;
mod settings;
mod results;
mod errors;
mod query;
use warp::Filter;
use std::convert::Infallible;
// use std::error;
use std::sync::Arc;
use pretty_env_logger;

#[macro_use] extern crate log;


async fn get_metrics(s: Arc<settings::Settings>) -> Result<impl warp::Reply, Infallible> {
    let result = query::query_targets(&s).await;
    Ok(result.to_string())
}

// async fn get_metrics() -> Result<impl warp::Reply, Box<dyn error::Error>> {
// async fn get_metrics() -> Result<impl warp::Reply, warp::Rejection> {
//     return Ok("metrics");
// }

pub async fn run() {
    pretty_env_logger::init();
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let s = Arc::new(settings::Settings::new().unwrap());
    info!("settings: {:?}", s.clone());
    let state = warp::any().map(move || s.clone());

    // let hello = warp::path!("hello" / String)
    //     .map(|name| format!("Hello, {}!", name));


    let metrics = warp::path("metrics").and(state).and_then(get_metrics);

    // let routes = hello.or(metrics);
    let routes = metrics;
    let server = warp::serve(routes).run(([0, 0, 0, 0], 3030));

    server.await;
    info!("Initialization Complete!");
}


// pub async fn run() {
//     pretty_env_logger::init();
//     // GET /hello/warp => 200 OK with body "Hello, warp!"
//     let s = settings::Settings::new().unwrap();
//     info!("settings: {:?}", s);
//     let hello = warp::path!("hello" / String)
//         .map(|name| format!("Hello, {}!", name));

//     let metrics = warp::path!("metrics")
//         .map(|| format!("metrics"));

//     let routes = hello.or(metrics);

//     let server = warp::serve(routes).run(([0, 0, 0, 0], 3030));

//     let res = tokio::join!(
//         server,
//         cron::run_crons(&s),
//     );

//     match res {
//         ((), Ok(_)) => println!("Complete"),
//         ((), Err(err)) => panic!("Error {:?}", err),
//     }
//     info!("Initialization Complete!");
//     return ();

//     // return Ok(());
// }
