use std::time::Duration;

use reqwest::Response;

pub trait Monitorable: std::fmt::Debug + Send + Sync {
    fn url(&self) -> &str;
    fn validate(&self, res: &Response, duration: &Duration) -> bool;
}

#[derive(Debug)]
pub struct StatusMonitorable {
    url: String,
    expected_status: u16,
}

impl StatusMonitorable {
    pub fn new(url: impl Into<String>, expected_status: u16) -> Self {
        Self {
            url: url.into(),
            expected_status,
        }
    }
}

impl Monitorable for StatusMonitorable {
    fn url(&self) -> &str {
        &self.url
    }

    fn validate(&self, res: &Response, _duration: &Duration) -> bool {
        res.status() == self.expected_status
    }
}

#[derive(Debug)]
pub struct DurationMonitorable {
    url: String,
    max_duration: Duration,
}

impl DurationMonitorable {
    pub fn new(url: impl Into<String>, max_duration: Duration) -> Self {
        Self {
            url: url.into(),
            max_duration,
        }
    }
}

impl Monitorable for DurationMonitorable {
    fn url(&self) -> &str {
        &self.url
    }

    fn validate(&self, _res: &Response, duration: &Duration) -> bool {
        duration < &self.max_duration
    }
}
