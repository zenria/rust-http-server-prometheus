extern crate prometheus;
mod httpserver;
mod promhelpers;

use prometheus::Registry;
use std::sync::Arc;

pub struct Context {
    metric_registry: Registry,
}

fn main() {
    let ctx = Context {
        metric_registry: Registry::new(),
    };

    httpserver::start_http_server(Arc::new(ctx));
}
