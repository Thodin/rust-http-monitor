use std::time::Duration;

use config::Config;
use executor::Executor;
use monitorable::{DurationMonitorable, Monitorable, StatusMonitorable};

mod config;
mod executor;
mod monitorable;
mod monitoring_result;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let monitorables = vec![
        Monitorable::Status(StatusMonitorable::new("https://docs.rs", 200)),
        Monitorable::Status(StatusMonitorable::new("https://google.com", 200)),
        Monitorable::Duration(DurationMonitorable::new(
            "https://google.de",
            Duration::from_millis(100),
        )),
    ];
    let config = Config { monitorables };

    let cycle_time = Duration::from_secs(5);

    let executor = Executor::new(config, cycle_time);
    executor.run().await?;

    Ok(())
}
