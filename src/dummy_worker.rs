use prometheus::Registry;
use promhelpers;
use std::thread;
use std::time::Duration;

pub fn launch_workers(registry: &Registry) {
    let counter = promhelpers::new_counter(&registry, "dummy_ops_count", "Dummy operations count");
    for _i in 1..10 {
        let cloned_counter = counter.clone();
        thread::spawn(move || loop {
            cloned_counter.inc();
            thread::sleep(Duration::from_millis(100));
        });
    }
}
