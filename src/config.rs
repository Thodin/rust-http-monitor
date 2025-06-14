use std::time::Duration;

use crate::monitorable::Monitorable;

pub struct Config {
    pub monitorables: Vec<Monitorable>,
    pub cycle_time: Duration,
}
