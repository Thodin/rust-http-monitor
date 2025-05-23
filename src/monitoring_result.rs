use std::{sync::Arc, time::Duration};

use crate::monitorable::Monitorable;

#[derive(Debug, Clone)]
pub struct MonitoringResult {
    pub success: bool,
    pub monitorable: Arc<dyn Monitorable>,
    pub duration: Duration,
}
