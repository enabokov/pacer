use std::time::SystemTime;

pub struct ThrottleOptions {
    pub max_num_calls: u16,
    pub period_seconds: u64,
}

pub struct Throttler {
    options: &'static ThrottleOptions,

    _call_count: u16,
    _since_time: SystemTime,
}

impl Throttler {
    pub fn new(options: &'static ThrottleOptions) -> Throttler {
        Throttler {
            _call_count: 0 as u16,
            _since_time: SystemTime::now(),
            options: options,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum StatusCode {
    Ok,
    Throttled,
    Error,
}

pub trait Throttle {
    fn throttle(&mut self) -> StatusCode;
}

impl<'a> Throttle for Throttler {
    fn throttle(&mut self) -> StatusCode {
        let past_time_in_secs: u64;
        {
            let current_time = SystemTime::now();
            past_time_in_secs = current_time
                .duration_since(self._since_time)
                .unwrap()
                .as_secs();
        }

        if past_time_in_secs > self.options.period_seconds {
            self._call_count = 0;
            self._since_time = SystemTime::now();
            println!("Unthrottled!");
            return StatusCode::Ok;
        }

        if self._call_count >= self.options.max_num_calls {
            println!("Throttled!");
            return StatusCode::Throttled;
        }

        self._call_count += 1;
        println!("Call count: {}", self._call_count);
        return StatusCode::Ok;
    }
}
