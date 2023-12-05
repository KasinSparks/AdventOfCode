use std::{time::{Instant, Duration}, fmt::Display};

pub struct Timer {
    name: String,
    start_time: Option<Instant>,
    duration: Duration,
}

impl Timer {
    pub fn new(name: &str) -> Self {
        return Timer {
            name: name.to_string(),
            start_time: None,
            duration: Duration::ZERO 
        };
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn end(&mut self) {
        if self.start_time.is_some() {
            self.duration = self.start_time.unwrap().elapsed();
        } else {
            self.duration = Duration::ZERO;
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_uppercase()
    }

    pub fn get_duration(&self) -> Duration {
        return self.duration;
    }
}

impl Display for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.write_str(format!("TIMER::{}: {:?}", self.get_name(), self.get_duration()).as_str());
    }
}