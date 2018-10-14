#[macro_use]
extern crate lazy_static;
extern crate prometheus;
mod dummy_worker;
mod httpserver;
mod promhelpers;

use prometheus::Registry;

pub struct Context {
    metric_registry: Registry,
}

lazy_static! {
    static ref CTX: Context = Context {
        metric_registry: Registry::new(),
    };
}

fn main() {
    dummy_worker::launch_workers(&CTX);

    httpserver::start_http_server(&CTX);
}
