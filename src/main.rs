extern crate prometheus;
mod dummy_worker;
mod httpserver;
mod promhelpers;

use prometheus::Registry;
use std::sync::Arc;

pub struct Context {
    metric_registry: Registry,
}

fn main() {
    let ctx = Arc::new(Context {
        metric_registry: Registry::new(),
    });

    dummy_worker::launch_workers(Arc::clone(&ctx));

    httpserver::start_http_server(ctx);
}
