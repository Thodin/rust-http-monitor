use std::time::Duration;

use reqwest::Response;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Monitorable {
    Status(StatusMonitorable),
    Duration(DurationMonitorable),
}

impl Monitorable {
    pub fn url(&self) -> &str {
        match self {
            Monitorable::Status(status_monitorable) => &status_monitorable.url,
            Monitorable::Duration(duration_monitorable) => &duration_monitorable.url,
        }
    }

    pub fn validate(&self, res: &Response, duration: &Duration) -> bool {
        match self {
            Monitorable::Status(status_monitorable) => {
                res.status() == status_monitorable.expected_status
            }
            Monitorable::Duration(duration_monitorable) => {
                duration < &duration_monitorable.max_duration
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMonitorable {
    url: String,
    expected_status: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurationMonitorable {
    url: String,
    max_duration: Duration,
}
