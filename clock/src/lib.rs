use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mH:i32 = minutes/60;
        let mM:i32 = minutes%60;
        let mut H:i32 = if mM < 0 { -1 } else { 0 };
        let M = (60 + mM) % 60;

        H = (((H + hours + mH) % 24) + 24) % 24;

        Clock {
            hours: H,
            minutes: M
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
