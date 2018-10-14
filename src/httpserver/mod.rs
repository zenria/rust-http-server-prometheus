extern crate futures;
extern crate hyper;

mod never;
mod service;

use self::futures::Future;
use self::hyper::Server;
use self::service::HttpService;

use super::Context;

pub fn start_http_server(ctx: &'static Context) {
    // This is our socket address...
    let addr = ([127, 0, 0, 1], 3000).into();

    let service = HttpService::new(&ctx);
    let server = Server::bind(&addr)
        .serve(service)
        .map_err(|e| eprintln!("server error: {}", e));

    // Run this server for... forever!
    hyper::rt::run(server);
}
