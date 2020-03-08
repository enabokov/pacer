use std::time::{SystemTime};

pub struct ThrottleEnforce {
    pub max_num_calls: u16,
    pub period_seconds: u64,
}

pub struct Throttler {
    called_count: u16,
    pinned_time: SystemTime,
    options: &'static ThrottleEnforce,
}

impl Throttler {
    pub fn new(t: &'static ThrottleEnforce) -> Throttler {
        Throttler {
            called_count: 0 as u16,
            pinned_time: SystemTime::now(),
            options: t,
        }
    } 
}

pub trait Throttle {
    fn throttle(&mut self);
}

impl<'a> Throttle for Throttler {
    fn throttle(&mut self) {
        if SystemTime::now()
            .duration_since(self.pinned_time).unwrap().as_secs() > self.options.period_seconds {
                self.called_count = 0;
                self.pinned_time = SystemTime::now()
        } else {
            if self.called_count >= self.options.max_num_calls {
                println!("Throttling");
                return;
            } else {
                self.called_count += 1;
                println!("Current call {}", self.called_count);                
            }
        }
    }
}
