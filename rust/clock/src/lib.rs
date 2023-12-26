use std::fmt::Display;

const MINUTES_PER_DAY: i32 = MINUTES_PET_HOUR * 24;
const MINUTES_PET_HOUR: i32 = 60;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minutes: (hours * MINUTES_PET_HOUR + minutes).rem_euclid(MINUTES_PER_DAY),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            minutes: (self.minutes + minutes).rem_euclid(MINUTES_PER_DAY),
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            self.minutes / MINUTES_PET_HOUR,
            self.minutes % MINUTES_PET_HOUR
        )
    }
}
