#[macro_use]
extern crate lazy_static;
extern crate prometheus;
mod dummy_worker;
mod httpserver;
mod promhelpers;

use prometheus::Registry;

pub struct Context {
    metric_registry: &'static Registry,
}

impl httpserver::MetricsRegistryProvider for CTX {
    fn get_metrics_registry(&'static self) -> &Registry {
        self.metric_registry
    }
}

lazy_static! {
    static ref REGISTRY: Registry = Registry::new();
    static ref CTX: Context = Context {
        metric_registry: &REGISTRY,
    };
}

fn main() {
    dummy_worker::launch_workers(CTX.metric_registry);

    httpserver::start_http_server(&CTX);
}
