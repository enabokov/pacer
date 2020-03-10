pub mod throttler;

#[cfg(test)]
mod tests {
    use super::throttler::{StatusCode, Throttle, ThrottleOptions, Throttler};

    #[test]
    fn check_throttle_options_creation() {
        //! create Throttle which allows 1 request within 5 seconds
        let options = ThrottleOptions {
            max_num_calls: 1,
            period_seconds: 5,
        };
        assert_eq!(options.max_num_calls, 1);
        assert_eq!(options.period_seconds, 5);
    }

    #[test]
    fn check_throttle_works() {
        let mut throttler = Throttler::new(&ThrottleOptions {
            max_num_calls: 2,
            period_seconds: 5,
        });
        let mut status_code: StatusCode = throttler.throttle();
        assert_eq!(status_code, StatusCode::Ok);

        status_code = throttler.throttle();
        assert_eq!(status_code, StatusCode::Ok);

        status_code = throttler.throttle();
        assert_eq!(status_code, StatusCode::Throttled);
    }
}
