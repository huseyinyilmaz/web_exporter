mod cron;
mod settings;
mod results;
use warp::Filter;
extern crate pretty_env_logger;

#[macro_use] extern crate log;

pub async fn run() {
    pretty_env_logger::init();
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let s = settings::Settings::new().unwrap();
    info!("settings: {:?}", s);
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let metrics = warp::path!("metrics")
        .map(|| format!("metrics"));

    let routes = hello.or(metrics);

    let server = warp::serve(routes).run(([0, 0, 0, 0], 3030));

    let res = tokio::join!(
        server,
        cron::run_crons(&s),
    );

    match res {
        ((), Ok(_)) => println!("Complete"),
        ((), Err(err)) => panic!("Error {:?}", err),
    }
    info!("Initialization Complete!");
    return ();

    // return Ok(());
}
