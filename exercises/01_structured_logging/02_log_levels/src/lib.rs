//! # Exercise
//!
//! We'll build a toy logger that supports filtering log records based on their level and
//! their source.
//!
//! Fill in the `todo!()`s as necessary in the `logger` module.

mod logger;

pub use logger::FilteredLogger;

pub mod one {
    pub fn work() {
        log::trace!("Starting to do something!");
        log::info!("Working really hard!");
        log::error!("Oh no, it failed!");
    }
}

pub mod two {
    pub fn work() {
        log::trace!("Wakey wakey!");
        log::info!("Time to do some work!");
        log::warn!("Almost done!");
    }
}
