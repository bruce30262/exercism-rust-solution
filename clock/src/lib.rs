use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let m_h:i32 = minutes/60;
        let m_m:i32 = minutes%60;
        let mut h:i32 = if m_m < 0 { -1 } else { 0 };
        let m = (60 + m_m) % 60;

        h = (((h + hours + m_h) % 24) + 24) % 24;

        Clock {
            hours: h,
            minutes: m
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
