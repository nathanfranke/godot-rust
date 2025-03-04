pub use anyhow::{Result, anyhow};
pub use log::info;

use env_logger::builder as env_logger;
use log::LevelFilter;

pub fn init() -> Result<()> {
    env_logger().filter_level(LevelFilter::Info).try_init()?;

    Ok(())
}

#[macro_export]
macro_rules! ok {
    ($expr:expr) => {
        match $expr {
            Ok(v) => v,
            Err(v) => {
                godot_error!("error: {v}");

                return Default::default();
            }
        }
    };
}
