use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct ScheduledTask {
    pub name: String,
    pub interval: Duration,
    pub last_run: Option<Instant>,
}

pub struct D0Automation;

impl D0Automation {
    pub fn create_task(name: &str, interval_secs: u64) -> ScheduledTask {
        ScheduledTask {
            name: name.to_string(),
            interval: Duration::from_secs(interval_secs),
            last_run: None,
        }
    }

    pub fn should_run(task: &ScheduledTask) -> bool {
        match task.last_run {
            None => true,
            Some(last) => last.elapsed() >= task.interval,
        }
    }
}
