use tokio::time::Duration;

#[derive(Default, Debug, Clone)]
pub struct TimeConfig {
    pub timeout: u64, 
}
