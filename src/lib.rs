mod cron;
use std::error;
#[macro_use] extern crate log;

pub async fn run() -> Result<(), Box<dyn error::Error>> {
    cron::crawl_targets().await;
    return Ok(());
}
