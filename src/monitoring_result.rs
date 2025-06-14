use std::{sync::Arc, time::Duration};

use crate::monitorable::Monitorable;

#[derive(Debug, Clone)]
pub struct MonitoringResult {
    pub success: bool,
    pub monitorable: Arc<Monitorable>,
    pub duration: Duration,
}
