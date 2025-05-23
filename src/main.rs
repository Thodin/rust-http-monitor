use std::{sync::Arc, time::Duration};

use executor::Executor;
use monitorable::{DurationMonitorable, Monitorable, StatusMonitorable};

mod executor;
mod monitorable;
mod monitoring_result;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let status_monitorables = vec![
        StatusMonitorable::new("https://docs.rs", 200),
        StatusMonitorable::new("https://google.com", 200),
    ];
    let duration_monitorables = vec![DurationMonitorable::new(
        "https://google.de",
        Duration::from_millis(100),
    )];
    let mut monitorables: Vec<_> = status_monitorables
        .into_iter()
        .map(|m| Arc::new(m) as Arc<dyn Monitorable>)
        .collect();

    let duration_monitorables: Vec<_> = duration_monitorables
        .into_iter()
        .map(|m| Arc::new(m) as Arc<dyn Monitorable>)
        .collect();

    monitorables.extend(duration_monitorables);

    let cycle_time = Duration::from_secs(5);

    let executor = Executor::new(monitorables, cycle_time);
    executor.run().await?;

    Ok(())
}
