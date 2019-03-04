#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate slog;
use slog_envlogger;

use actix;
use actix_web::{server, App, HttpRequest, Responder};

mod api;
mod config;
mod logging;
mod model;
mod vizceral;

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    ::std::env::set_var("RUST_BACKTRACE", "1");
    let _guard = slog_envlogger::init().unwrap();
    let logger = logging::root_logger();

    let sys = actix::System::new("observatory");
    let service_logger = logger.clone();
    server::new(move || {
        let logger = service_logger.clone();
        api::get_app(logger)
    })
    .bind("127.0.0.1:8081")
    .unwrap()
    .shutdown_timeout(1)
    .start();

    info!(logger, "Server started on 127.0.0.1:8081");
    let _ = sys.run();
}
