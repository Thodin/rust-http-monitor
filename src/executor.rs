use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use tokio::{select, task::JoinSet};

use crate::{config::Config, monitorable::Monitorable, monitoring_result::MonitoringResult};

pub struct Executor {
    monitorables: Vec<Arc<Monitorable>>,
    cycle_time: Duration,
}

impl Executor {
    pub fn new(config: Config) -> Self {
        let monitorables = config.monitorables.into_iter().map(Arc::new).collect();

        Self {
            monitorables,
            cycle_time: config.cycle_time,
        }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let mut interval = tokio::time::interval(self.cycle_time);
        loop {
            interval.tick().await;
            let mut task_set = JoinSet::new();

            for m in &self.monitorables {
                task_set.spawn(monitor(m.clone(), self.cycle_time));
            }

            let results = task_set.join_all().await;
            dbg!(results);
        }
    }
}

async fn monitor(
    monitorable: Arc<Monitorable>,
    timeout: Duration,
) -> anyhow::Result<MonitoringResult> {
    let timeout_task = tokio::time::sleep(timeout);
    let request = reqwest::get(monitorable.url());

    let now = Instant::now();
    select! {
        _ = timeout_task => {
            println!("Request to {} timed out (timeout {:?})", monitorable.url(), timeout);
            Ok(MonitoringResult { success: false, monitorable, duration: timeout })
        },
        res = request => {
            let elapsed = now.elapsed();
            let Ok(res) = res else {
                return Ok(MonitoringResult { success: false, monitorable, duration: Duration::ZERO });
            };

            let success = monitorable.validate(&res, &elapsed);
            Ok(MonitoringResult {
                success,
                monitorable,
                duration: elapsed,
            })
        }
    }
}
