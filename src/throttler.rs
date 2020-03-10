use std::time::SystemTime;

pub struct ThrottleOptions {
    pub max_call_count: u16,
    pub period_seconds: u64,
}

#[derive(PartialEq, Debug)]
pub enum StatusCode {
    Ok,
    Throttled,
    Error,
}

pub struct Throttler<'a> {
    options: &'a ThrottleOptions,

    _call_count: u16,
    _since_time: SystemTime,
}

impl<'a> Throttler<'a> {
    pub fn new(options: &'a ThrottleOptions) -> Throttler {
        Throttler {
            _call_count: 0,
            _since_time: SystemTime::now(),
            options: options,
        }
    }

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
