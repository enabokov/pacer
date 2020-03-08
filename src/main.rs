extern crate pacer;

use pacer::adjust::adjust::{Throttle, ThrottleEnforce, Throttler};

fn main() {
    let mut throttler = Throttler::new(&ThrottleEnforce {
        max_num_calls: 5,
        period_seconds: 5,
    });

    for _ in 0..50 {
        throttler.throttle();
        // std::thread::sleep(std::time::Duration::from_secs(5))
    }
}
