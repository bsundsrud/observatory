use chrono;
use slog::*;
use std::io;
use std::sync::atomic::{AtomicUsize, Ordering};

mod web;

pub use web::RequestLogger;

lazy_static! {
    static ref LEVEL: AtomicUsize = AtomicUsize::new(Level::Info.as_usize());
}

pub fn set_global_level(level: Level) {
    LEVEL.store(level.as_usize(), Ordering::SeqCst);
}

fn timestamp_fn(io: &mut io::Write) -> io::Result<()> {
    write!(
        io,
        "{}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f")
    )
}

pub fn root_logger() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator)
        .use_custom_timestamp(timestamp_fn)
        .build()
        .fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let drain = Filter::new(drain, |r: &Record| {
        let level = LEVEL.load(Ordering::Relaxed);
        let level = Level::from_usize(level).expect("Got an invalid level usize somehow");
        r.level().is_at_least(level)
    });
    Logger::root(drain.fuse(), o!())
}
