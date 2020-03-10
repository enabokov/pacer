use std::time::SystemTime;

pub struct ThrottleOptions {
    pub max_num_calls: u16,
    pub period_seconds: u64,
}

pub struct Throttler {
    _call_times: u16,
    _since_time: SystemTime,
    options: &'static ThrottleOptions,
}

impl Throttler {
    pub fn new(t: &'static ThrottleOptions) -> Throttler {
        Throttler {
            _call_times: 0 as u16,
            _since_time: SystemTime::now(),
            options: t,
        }
    }
}

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
            self._call_times = 0;
            self._since_time = SystemTime::now();
            return StatusCode::Ok;
        }

        if self._call_times >= self.options.max_num_calls {
            println!("Throttled!");
            return StatusCode::Throttled;
        } 

        self._call_times += 1;
        println!("Current call {}", self._call_times);
        return StatusCode::Ok;
    }
}
