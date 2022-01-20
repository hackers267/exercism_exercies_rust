use std::fmt::{Display, Formatter};

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_on_the_hour() {
        assert_eq!(Clock::new(8, 0).to_string(), "08:00")
    }
    #[test]
    fn test_past_the_hour() {
        assert_eq!(Clock::new(11, 9).to_string(), "11:09");
    }
}

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (hour, minute) = div_mod(minutes, 60);
        Self {
            hours: self.hours + hour,
            minutes: self.minutes + minute,
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

fn div_mod(dividend: i32, divisor: i32) -> (i32, i32) {
    (dividend / divisor, dividend % divisor)
}
