use std::time::Duration;

use executor::Executor;
use monitorable::{DurationMonitorable, Monitorable, StatusMonitorable};

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

    let cycle_time = Duration::from_secs(5);

    let executor = Executor::new(monitorables, cycle_time);
    executor.run().await?;

    Ok(())
}
