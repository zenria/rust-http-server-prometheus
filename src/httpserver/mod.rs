
mod never;
mod service;

use futures::Future;
use hyper::Server;
use hyper::rt::run;
use self::service::HttpService;

use prometheus::Registry;

pub trait MetricsRegistryProvider {
    fn get_metrics_registry(&'static self) -> &Registry;
}

pub fn start_http_server(ctx: &'static (MetricsRegistryProvider + Sync + Send)) {
    // This is our socket address...
    let addr = ([127, 0, 0, 1], 3000).into();

    let service = HttpService::new(ctx);
    let server = Server::bind(&addr)
        .serve(service)
        .map_err(|e| eprintln!("server error: {}", e));

    // Run this server for... forever!
    run(server);
}
