extern crate pacer;

use pacer::adjust::throttler::{StatusCode, Throttle, ThrottleOptions, Throttler};

fn main() {
    let mut throttler = Throttler::new(&ThrottleOptions {
        max_num_calls: 1,
        period_seconds: 2,
    });

    for _ in 0..50 {
        let status_code = throttler.throttle();
        // println!("Current status {}", status_code);
        match status_code {
            StatusCode::Ok => {}
            StatusCode::Throttled => std::thread::sleep(std::time::Duration::from_secs(6)),
            StatusCode::Error => println!("Error!"),
        }
    }
}
