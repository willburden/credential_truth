//! Logger set up - using env-logger, attempting to match the style
//! of clap for the sake of consistency.

use std::io::Write;

use log::LevelFilter;

/// Initialises the logger.
/// 
/// If LOG_LEVEL is set at runtime, it will use that log level.
/// 
/// Otherwise, if LOG_LEVEL was set at *compile*-time, it will use
/// *that* log level.
/// 
/// If neither of those are true, it will use a default log level, debug.
pub fn init_logger() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Debug)
        .parse_filters(option_env!("LOG_LEVEL").unwrap_or(""))
        .parse_env("LOG_LEVEL")
        .format(|buf, record| {
            let style = buf.default_level_style(record.level());
            writeln!(
                buf, "{} {}",
                style.value(format!("{}:", record.level().as_str().to_lowercase())),
                record.args()
            )
            // writeln!(f, "{}: {}", record.level().as_str().to_lowercase(), r.args())
        })
        .init();
}
