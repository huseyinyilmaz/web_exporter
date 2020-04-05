use warp::Filter;
extern crate pretty_env_logger;
#[macro_use] extern crate log;
use web_exporter::run;
// se std::error;
// use std::result::Result;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let metrics = warp::path!("metrics")
        .map(|| format!("metrics"));

    let routes = hello.or(metrics);

    let server = warp::serve(routes).run(([0, 0, 0, 0], 3030));

    let res = tokio::join!(
        server,
        run(),
    );

    match res {
        ((), Ok(_)) => println!("Complete"),
        ((), Err(err)) => panic!("Error {:?}", err),
    }
    info!("Hello, world!");
    return ();
}
