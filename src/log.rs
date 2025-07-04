use tracing::Level;
pub use tracing::{debug, error, info, warn};

const DEBUG: &str = "debug";

pub struct InitParams {
    log_level: Level,
    without_time: bool,
}

impl Default for InitParams {
    fn default() -> Self {
        Self::new()
    }
}

impl InitParams {
    pub fn new() -> Self {
        InitParams {
            log_level: Level::INFO,
            without_time: true,
        }
    }
    pub fn set_log_level(&mut self, level: &str) {
        self.log_level = match level {
            DEBUG => Level::DEBUG,
            _ => Level::INFO,
        }
    }
    pub fn set_without_time(&mut self, without_time: bool) {
        self.without_time = without_time;
    }
}

pub fn log_init(param: &InitParams) {
    // Common formatter configuration
    let base = tracing_subscriber::fmt()
        .with_max_level(param.log_level)
        .with_line_number(true);

    // Build the subscriber depending on the `without_time` flag, boxing it so
    // both branches unify to the same type.
    let subscriber: Box<dyn tracing::Subscriber + Send + Sync> = if param.without_time {
        Box::new(base.without_time().finish())
    } else {
        Box::new(base.finish())
    };

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    info!("Log level is {}", param.log_level.as_str());
}

#[cfg(test)]
mod tests {
    #[test]
    fn init_log() {
        let init_params = super::InitParams::default();
        super::log_init(&init_params);
        assert_eq!(init_params.log_level, tracing::Level::INFO);
    }
}
