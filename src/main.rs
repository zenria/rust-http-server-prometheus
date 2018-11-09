extern crate prometheus;
extern crate futures;
extern crate hyper;

mod dummy_worker;
mod httpserver;
mod promhelpers;



fn main() {
    dummy_worker::launch_workers();

    httpserver::start_http_server();
}
