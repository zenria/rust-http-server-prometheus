
mod never;
mod service;

use futures::Future;
use hyper::Server;
use hyper::rt::run;
use self::service::HttpService;



pub fn start_http_server() {
    // This is our socket address...
    let addr = ([127, 0, 0, 1], 3000).into();

    let service = HttpService::new();
    let server = Server::bind(&addr)
        .serve(service)
        .map_err(|e| eprintln!("server error: {}", e));

    // Run this server for... forever!
    run(server);
}
