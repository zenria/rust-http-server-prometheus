use prometheus::{Counter, Opts, Registry};

pub fn new_counter(registry: &Registry, counter_name: &str, counter_help: &str) -> Counter {
    let counter = Counter::with_opts(Opts::new(counter_name, counter_help)).unwrap();
    registry.register(Box::new(counter.clone())).unwrap();
    counter
}
