//! This is documentation for the `throttler` module
//!
//! # Examples
//!
//! ```
//! use pacer::throttler::{StatusCode, ThrottleOptions, Throttler};
//!
//! fn main() {
//!     let mut throttler = Throttler::new(&ThrottleOptions {
//!         max_call_count: 2,
//!         period_seconds: 5,
//!     });
//!
//!     request(&mut throttler);
//! }
//!
//! fn request(throttler: &mut Throttler) {
//!     let status_code: StatusCode = throttler.throttle();
//!     match status_code {
//!         StatusCode::Ok => { println!("Request can be operated") },
//!         StatusCode::Error => { println!("Something went wrong!") },
//!         StatusCode::Throttled => { println!("It's throttled!") },
//!     };
//! }
//! ```

use std::time::SystemTime;

/// The `ThrottleOptions` struct.
pub struct ThrottleOptions {
    /// Max. number of calls
    pub max_call_count: u16,
    /// Period in seconds
    /// of the max. number of the calls
    /// can be performed
    pub period_seconds: u64,
}

/// The `StatusCode` enum.
/// 
/// Used to return status of the throttle function.
#[derive(PartialEq, Debug)]
pub enum StatusCode {
    /// Calls can be performed.
    Ok,
    /// Calls are throttled.
    /// Required to wait.
    Throttled,
    /// Throttle goes wrong.
    /// Something does not work properly.
    Error,
}

/// The `Throttler` struct.
/// 
/// The main struct that is used to enable throttling.
/// It implements 2 methods:
/// - new
/// - throttle
/// 
pub struct Throttler<'a> {
    /// Each Throttler has options to set.
    options: &'a ThrottleOptions,
    /// How many times it is called
    _call_count: u16,
    /// Since the time a last call was throttled
    _since_time: SystemTime,
}

impl<'a> Throttler<'a> {
    /// Create an instance of the Throttler
    /// with its configured options
    /// and set initial values
    pub fn new(options: &'a ThrottleOptions) -> Throttler {
        Throttler {
            _call_count: 0,
            _since_time: SystemTime::now(),
            options: options,
        }
    }

    /// Call `throttle` to limit traffic.
    /// e.g it can be called within HTTP requests.
    pub fn throttle(&mut self) -> StatusCode {
        let past_time_in_secs: u64;
        {
            let current_time = SystemTime::now();
            past_time_in_secs = current_time
                .duration_since(self._since_time)
                .unwrap()
                .as_secs();
        }

        if past_time_in_secs >= self.options.period_seconds {
            self._call_count = 0;
            self._since_time = SystemTime::now();
            println!("Unthrottled!");
            return StatusCode::Ok;
        }

        if self._call_count >= self.options.max_call_count {
            println!("Throttled!");
            return StatusCode::Throttled;
        }

        self._call_count += 1;
        println!("Call count: {}", self._call_count);
        return StatusCode::Ok;
    }
}
