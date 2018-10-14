use promhelpers;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use Context;

pub fn launch_worker(ctx: Arc<Context>) {
    let counter = promhelpers::new_counter(&ctx, "dummy_ops_count", "Dummy operations count");

    thread::spawn(move || loop {
        counter.inc();
        thread::sleep(Duration::from_millis(100));
    });
}
