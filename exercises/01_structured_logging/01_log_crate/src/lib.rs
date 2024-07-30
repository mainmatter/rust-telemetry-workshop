use log::{LevelFilter, Log, Record};
use std::io::Write;
use std::sync::Mutex;

/// The logic in our program hasn't changed: we're still taking a list of arguments, expecting
/// at least two of them, and logging out their space-concatenated values.
/// We'll be invoking this program from two different CLIs (`stdout.rs` and `file.rs`),
/// each using a different logging configuration—a perfect opportunity to see how the `log` crate
/// can help us decouple the instrumentation API from the processing code.
///
/// # Exercise
///
/// Replace all the `todo!()` calls with the appropriate `log` macro invocation to get the
/// tests to pass.
pub fn entrypoint(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    todo!();
    let Some(a) = args.get(0) else {
        return Err("You haven't passed any argument to the program! Two is the minimum.".into());
    };
    todo!();
    let Some(b) = args.get(1) else {
        return Err(
            "You have only passed one argument to the program, you need another one!".into(),
        );
    };

    todo!();

    Ok(())
}

/// A logger implementation that emits the logged message to the chosen sink.
///
/// In the real world, you're likely to use a ready-made implementation from the Rust crate
/// ecosystem. Many high-quality options are listed in the documentation of `log` itself.
///
/// We are providing a simple implementation here as a learning opportunity.
pub struct SimpleLogger<Sink>(Mutex<Sink>);

impl<Sink> SimpleLogger<Sink>
where
    // We need to be able to:
    // - write to the sink (`Write` trait)
    // - send it across threads (`Send` trait)
    // - share it across threads (`Sync` trait)
    // - use the sink for as long as the program runs (`'static` lifetime)
    //
    // The last three requirements come from the `log::Log` trait itself.
    Sink: Write + Send + Sync + 'static,
{
    pub fn init(sink: Sink) -> Result<(), log::SetLoggerError> {
        // We need to wrap the sink in a `Mutex` since logs could be emitted from multiple threads.
        // We use a lock to ensure that only one thread at a time can write to the sink.
        let logger = Self(Mutex::new(sink));
        // We need to "install" the logger in order to start piping log records through its processing
        // logic.
        // Tip: use the `set_boxed_logger` function.
        todo!();

        // We'll talk about levels in the next exercise, don't worry!
        log::set_max_level(LevelFilter::Trace);

        Ok(())
    }
}

/// All loggers for the `log` crate must implement the `Log` trait.
/// It determines how the messages emitted via the instrumentation API (i.e. `log`'s macros)
/// will be processed.
///
/// Our implementation ignores the failure cases for now, but we'll improve upon that later in the
/// course.
impl<Sink> Log for SimpleLogger<Sink>
where
    Sink: Write + Send + Sync,
{
    fn log(&self, record: &Record) {
        // We try to emit the log message to the chosen sink.
        // This operation *could* fail—e.g. the sink is a file and the disk is full.
        if let Ok(mut sink) = self.0.lock() {
            // Tip: checkout `writeln!` in the standard library documentation.
            todo!()
        }
    }

    fn flush(&self) {
        // Some sinks may buffer log messages in memory before writing them to their final
        // destination. The `flush` method is used to force the sink to write any buffered data
        // immediately.
        if let Ok(mut sink) = self.0.lock() {
            let _ = sink.flush();
        }
    }

    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        // We'll be talking about this soon enough, ignore it for now!
        true
    }
}
