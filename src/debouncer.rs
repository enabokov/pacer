//! This is documentation for the `debouncer` module
//!
//! # Examples
//!
//! ```
//! use pacer::debouncer::{StatusCode, DebounceOptions, Debouncer};
//!
//! fn main() {
//!     let mut debouncer = Debouncer::new(&DebounceOptions {
//!         period_seconds: 5,
//!     });
//!
//!     request(&mut debouncer);
//! }
//!
//! fn request(debouncer: &mut Debouncer) {
//!     let status_code: StatusCode = debouncer.debounce();
//!     match status_code {
//!         StatusCode::Ok => { println!("Request can be operated") },
//!         StatusCode::Error => { println!("Something went wrong!") },
//!         StatusCode::Debounced => { println!("It's debounced!") },
//!     };
//! }
//! ```

use std::time::SystemTime;

/// The `DebounceOptions` struct.
pub struct DebounceOptions {
    /// how long to wait
    /// before proceed with request
    pub period_seconds: u64,
}

/// The `StatusCode` enum.
///
/// Used to return status of the debounce function.
#[derive(PartialEq, Debug)]
pub enum StatusCode {
    /// Calls can be performed
    Ok,
    /// Calls are debounced
    /// Required to stop requesting and wait some time
    Debounced,
    /// Debounce went wrong.
    /// Something does not work properly.
    Error,
}

/// The `Debouncer` struct.
///
/// The main struct that is used to enable throttling.
/// It implements 2 methods:
/// - new
/// - throttle
///
pub struct Debouncer<'a> {
    /// Each Throttler has options to set.
    options: &'a DebounceOptions,
    /// Since the time a last call was throttled
    _last_request: SystemTime,
}

impl<'a> Debouncer<'a> {
    /// Create an instance of the Debouncer
    /// with its configured options
    /// and set initial values
    pub fn new(options: &'a DebounceOptions) -> Debouncer {
        Debouncer {
            options: options,
            _last_request: SystemTime::now(),
        }
    }

    /// Call `debounce` to limit traffic.
    /// e.g it can be called within HTTP requests.
    pub fn debounce(&mut self) -> StatusCode {
        let past_time_in_secs: u64;
        {
            let current_time = SystemTime::now();
            past_time_in_secs = current_time
                .duration_since(self._last_request)
                .unwrap()
                .as_secs();
        }

        if past_time_in_secs < self.options.period_seconds {
            self._last_request = SystemTime::now();
            println!("Debounced!");
            return StatusCode::Debounced;
        }

        self._last_request = SystemTime::now();
        return StatusCode::Ok;
    }
}
