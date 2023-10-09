use log::{LevelFilter, Metadata, Record};
use std::cmp::max;
use std::collections::HashMap;

/// A logger implementation that filters log records based on their level and the module they come
/// from.
pub struct FilteredLogger {
    default_level_filter: LevelFilter,
    /// If a record comes from one of the modules in this map, we apply the corresponding filter
    /// instead of the default one.
    module_filters: HashMap<String, LevelFilter>,
}

impl FilteredLogger {
    pub fn init(
        default_level_filter: LevelFilter,
        module_filters: HashMap<String, LevelFilter>,
    ) -> Result<(), log::SetLoggerError> {
        // A word of caution: a `Level` is allowed by a `LevelFilter` if the `Level` is
        // **smaller** than or equal to the `LevelFilter`.
        // Therefore a greater `LevelFilter` is a **more permissive** one.
        // A bit twisted, yes.
        //
        // In order to allow specific modules to be logged at a more verbose level than the default
        // one, we need to find the maximum level filter among all the module filters.
        // We'll use this as the overall maximum level for the logger.
        let max_level = max(
            module_filters
                .values()
                .copied()
                .max()
                .unwrap_or(default_level_filter),
            default_level_filter,
        );

        let logger = Self {
            default_level_filter,
            module_filters,
        };

        log::set_boxed_logger(Box::new(logger))?;
        log::set_max_level(max_level);
        Ok(())
    }
}

impl log::Log for FilteredLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        // I'll repeat it again: a `Level` is allowed by a `LevelFilter` if the `Level` is
        // **smaller than or equal to** the `LevelFilter`.
        // Therefore a greater `LevelFilter` is a **more permissive** one.
        // A bit twisted, yes.
        //
        // Check if we have a module-specific filter for this record, otherwise use the
        // default one.
        if let Some(module_filter) = self.module_filters.get(metadata.target()) {
            metadata.level() <= *module_filter
        } else {
            metadata.level() <= self.default_level_filter
        }
    }

    fn log(&self, record: &Record) {
        // We need to explicitly check if the record should be emitted.
        // The `log` crate won't do it automatically for us!
        if self.enabled(record.metadata()) {
            // We log straight to stdout in this example, for simplicity.
            println!("{}", record.args());
        }
    }

    fn flush(&self) {}
}
