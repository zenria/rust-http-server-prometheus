use hyper::service::{NewService, Service};
use hyper::{Body, Method, Request, Response, StatusCode};

use futures::future;
use futures::IntoFuture;
use super::never::Never;

use super::MetricsRegistryProvider;
use prometheus::{Counter, Encoder, TextEncoder};
use promhelpers;
use std::sync::Arc;

struct Metrics {
    request_count: Counter,
    error_count: Counter,
    slash_metrics_count: Counter,
}

pub struct HttpService {
    ctx: &'static (MetricsRegistryProvider + Send + Sync),
    metrics: Arc<Metrics>,
}

impl HttpService {
    pub fn new(ctx: &'static (MetricsRegistryProvider + Send + Sync)) -> HttpService {
        // Create a Counter.
        let metrics = Metrics {
            request_count: promhelpers::new_counter(
                ctx.get_metrics_registry(),
                "request_count",
                "Served http requests count",
            ),
            error_count: promhelpers::new_counter(
                ctx.get_metrics_registry(),
                "error_count",
                "HTTP errors count",
            ),
            slash_metrics_count: promhelpers::new_counter(
                ctx.get_metrics_registry(),
                "slash_metrics_count",
                "/metrics request count",
            ),
        };
        HttpService {
            ctx: ctx,
            metrics: Arc::new(metrics),
        }
    }
}

/**
 * Be simple: the service also impl NewService
 *
 */
impl NewService for HttpService {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = Never;
    type Service = HttpService;

    type Future = future::FutureResult<HttpService, Never>;
    type InitError = Never;

    fn new_service(&self) -> Self::Future {
        self.clone().into_future()
    }
}

impl IntoFuture for HttpService {
    type Future = future::FutureResult<HttpService, Never>;
    type Item = HttpService;
    type Error = Never;

    fn into_future(self) -> <Self as IntoFuture>::Future {
        future::ok(self)
    }
}

impl Clone for HttpService {
    fn clone(&self) -> HttpService {
        HttpService {
            ctx: self.ctx,
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
            (&Method::GET, "/") | (&Method::GET, "/index.html") => Response::builder()
                .header("Content-Type", "text/plain")
                .body(Body::from(PHRASE))
                .unwrap(),
            (&Method::GET, "/metrics") => {
                self.metrics.slash_metrics_count.inc();
                Response::builder()
                    .header("Content-Type", "text/plain")
                    .body(Body::from(self.produce_metrics()))
                    .unwrap()
            }
            _ => {
                self.metrics.error_count.inc();
                // Return 404 not found response.
                let body = Body::from(NOT_FOUND);
                Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .header("Content-Type", "text/plain")
                    .body(body)
                    .unwrap()
            }
        }
    }

    fn produce_metrics(&self) -> String {
        let registry = self.ctx.get_metrics_registry();
        // Gather the metrics.
        let mut buffer = vec![];
        let encoder = TextEncoder::new();
        let metric_families = registry.gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();

        String::from_utf8(buffer).unwrap()
    }
}

impl Service for HttpService {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = Never;
    type Future = future::FutureResult<Response<Body>, Never>;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        future::ok(self.serve(req))
    }
}
