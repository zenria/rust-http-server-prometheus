use promhelpers;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use Context;

pub fn launch_workers(ctx: Arc<Context>) {
    let counter = promhelpers::new_counter(&ctx, "dummy_ops_count", "Dummy operations count");
    for _i in 1..20 {
        let cloned_counter = counter.clone();
        thread::spawn(move || loop {
            cloned_counter.inc();
            thread::sleep(Duration::from_millis(10));
        });
    }
}
