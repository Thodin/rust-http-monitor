use std::time::Duration;

use anyhow::Context;
use config::Config;
use executor::Executor;
use monitorable::Monitorable;

mod config;
mod executor;
mod monitorable;
mod monitoring_result;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Read monitorables from file.
    let file =
        std::fs::File::open("./monitorables.json").context("reading monitorables from file")?;
    let monitorables: Vec<Monitorable> =
        serde_json::from_reader(file).context("Parsing monitorables from json")?;

    let config = Config { monitorables };

    let cycle_time = Duration::from_secs(5);

    // Create and run the executor.
    let executor = Executor::new(config, cycle_time);
    executor.run().await?;

    Ok(())
}
