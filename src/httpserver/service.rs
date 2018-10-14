use super::hyper::service::{NewService, Service};
use super::hyper::{Body, Method, Request, Response, StatusCode};

use super::futures::future;
use super::never::Never;

use prometheus::{Counter, Encoder, Opts, TextEncoder};
use std::sync::Arc;
use Context;

struct Metrics {
    request_count: Counter,
    error_count: Counter,
    slash_metrics_count: Counter,
}

pub struct HttpService {
    ctx: Arc<Context>,
    metrics: Arc<Metrics>,
}

impl HttpService {
    fn new_counter(ctx: &Arc<Context>, counter_name: &str, counter_help: &str) -> Counter {
        let counter = Counter::with_opts(Opts::new(counter_name, counter_help)).unwrap();
        ctx.metric_registry
            .register(Box::new(counter.clone()))
            .unwrap();
        counter
    }
    pub fn new(ctx: &Arc<Context>) -> HttpService {
        // Create a Counter.
        let metrics = Metrics {
            request_count: HttpService::new_counter(
                ctx,
                "request_count",
                "Served http requests count",
            ),
            error_count: HttpService::new_counter(ctx, "error_count", "HTTP errors count"),
            slash_metrics_count: HttpService::new_counter(
                ctx,
                "slash_metrics_count",
                "/metrics request count",
            ),
        };
        HttpService {
            ctx: Arc::clone(ctx),
            metrics: Arc::new(metrics),
        }
    }
}

/**
 * Be simple: the service also impl NewService
 *
 */
impl NewService for HttpService {
    type ResBody = Body;
    type ReqBody = Body;
    type Future = future::FutureResult<HttpService, Never>;
    type Error = Never;

    type Service = HttpService;
    type InitError = Never;

    fn new_service(&self) -> Self::Future {
        future::ok(self.clone())
    }
}

impl Clone for HttpService {
    fn clone(&self) -> HttpService {
        HttpService {
            ctx: Arc::clone(&self.ctx),
            metrics: Arc::clone(&self.metrics),
        }
    }
}

const PHRASE: &str = "Hello, World!";
const NOT_FOUND: &str = "404 NOT FOUND";

impl HttpService {
    fn serve(&self, req: Request<Body>) -> Response<Body> {
        // Increment counter for each request
        self.metrics.request_count.inc();

        match (req.method(), req.uri().path()) {
            (&Method::GET, "/") | (&Method::GET, "/index.html") => {
                Response::new(Body::from(PHRASE))
            }
            (&Method::GET, "/metrics") => {
                self.metrics.slash_metrics_count.inc();

                Response::new(Body::from(self.produce_metrics()))
            }
            _ => {
                self.metrics.error_count.inc();
                // Return 404 not found response.
                let body = Body::from(NOT_FOUND);
                Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(body)
                    .unwrap()
            }
        }
    }

    fn produce_metrics(&self) -> String {
        let registry = &self.ctx.metric_registry;
        // Gather the metrics.
        let mut buffer = vec![];
        let encoder = TextEncoder::new();
        let metric_familys = registry.gather();
        encoder.encode(&metric_familys, &mut buffer).unwrap();

        String::from_utf8(buffer).unwrap()
    }
}

impl Service for HttpService {
    type ResBody = Body;
    type ReqBody = Body;
    type Future = future::FutureResult<Response<Body>, Never>;
    type Error = Never;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        future::ok(self.serve(req))
    }
}
