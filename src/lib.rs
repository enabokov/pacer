pub mod debouncer;
pub mod throttler;

#[cfg(test)]
mod tests {
    use super::debouncer::{DebounceOptions, Debouncer};
    use super::throttler::{ThrottleOptions, Throttler};

    #[test]
    fn check_throttle_options_creation() {
        //! create Throttle which allows 1 request within 5 seconds
        let options = ThrottleOptions {
            max_call_count: 1,
            period_seconds: 5,
        };
        assert_eq!(options.max_call_count, 1);
        assert_eq!(options.period_seconds, 5);
    }

    #[test]
    fn check_debounce_options_creation() {
        //! create Debounce which refuses all requests within 5 seconds
        let options = DebounceOptions { period_seconds: 5 };
        assert_eq!(options.period_seconds, 5);
    }

    #[test]
    fn check_throttle_work() {
        let mut throttler = Throttler::new(&ThrottleOptions {
            max_call_count: 2,
            period_seconds: 5,
        });

        let mut status_code: super::throttler::StatusCode = throttler.throttle();
        assert_eq!(status_code, super::throttler::StatusCode::Ok);

        status_code = throttler.throttle();
        assert_eq!(status_code, super::throttler::StatusCode::Ok);

        status_code = throttler.throttle();
        assert_eq!(status_code, super::throttler::StatusCode::Throttled);

        std::thread::sleep(std::time::Duration::from_secs(5));

        status_code = throttler.throttle();
        assert_eq!(status_code, super::throttler::StatusCode::Ok);
    }

    #[test]
    fn check_debounce_work() {
        let mut debouncer = Debouncer::new(&DebounceOptions { period_seconds: 5 });

        let mut status_code: super::debouncer::StatusCode = debouncer.debounce();
        assert_eq!(status_code, super::debouncer::StatusCode::Debounced);

        status_code = debouncer.debounce();
        assert_eq!(status_code, super::debouncer::StatusCode::Debounced);

        status_code = debouncer.debounce();
        assert_eq!(status_code, super::debouncer::StatusCode::Debounced);

        std::thread::sleep(std::time::Duration::from_secs(5));

        status_code = debouncer.debounce();
        assert_eq!(status_code, super::debouncer::StatusCode::Ok);
    }
}
