use prometheus::{Counter, Opts};
use Context;

pub fn new_counter(ctx: &Context, counter_name: &str, counter_help: &str) -> Counter {
    let counter = Counter::with_opts(Opts::new(counter_name, counter_help)).unwrap();
    ctx.metric_registry
        .register(Box::new(counter.clone()))
        .unwrap();
    counter
}
