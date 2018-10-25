#[macro_use]
extern crate lazy_static;
extern crate prometheus;
extern crate futures;
extern crate hyper;

mod dummy_worker;
mod httpserver;
mod promhelpers;

use prometheus::Registry;

impl httpserver::MetricsRegistryProvider for REGISTRY {
    fn get_metrics_registry(&'static self) -> &Registry {
        self
    }
}

lazy_static! {
    static ref REGISTRY: Registry = Registry::new();
}

fn main() {
    dummy_worker::launch_workers(&REGISTRY);

    httpserver::start_http_server(&REGISTRY);
}
