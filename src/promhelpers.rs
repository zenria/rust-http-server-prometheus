use prometheus::{Counter, Opts, register};

pub fn new_counter( counter_name: &str, counter_help: &str) -> Counter {
    let counter = Counter::with_opts(Opts::new(counter_name, counter_help)).unwrap();
    register(Box::new(counter.clone())).unwrap();
    counter
}
