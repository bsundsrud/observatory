use actix_web::error::Result as ActixResult;
use actix_web::middleware::{Finished, Middleware, Started};
use actix_web::{HttpRequest, HttpResponse};
use slog::Logger;
use std::time::Instant;

pub struct RequestLogger {
    log: Logger,
}

impl RequestLogger {
    pub fn new(log: Logger) -> RequestLogger {
        RequestLogger { log }
    }
}

struct StartTime(Instant);

impl<S> Middleware<S> for RequestLogger {
    fn start(&self, req: &HttpRequest<S>) -> ActixResult<Started> {
        req.extensions_mut().insert(StartTime(Instant::now()));
        Ok(Started::Done)
    }

    fn finish(&self, req: &HttpRequest<S>, resp: &HttpResponse) -> Finished {
        if let Some(entry_time) = req.extensions().get::<StartTime>() {
            let elapsed = entry_time.0.elapsed();
            let remote = if let Some(remote) = req.connection_info().remote() {
                format!("{}", remote)
            } else {
                "-".to_string()
            };
            let path = format!("{} {}", req.method(), req.path());
            let status = resp.status().as_u16();
            info!(
                self.log,
                "{} {} => {} {} {:?}",
                remote,
                path,
                status,
                resp.response_size(),
                elapsed
            )
        }
        Finished::Done
    }
}
