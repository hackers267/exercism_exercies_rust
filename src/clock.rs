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

    #[test]
    fn test_midnight_is_zero_hours() {
        assert_eq!(Clock::new(24, 0).to_string(), "00:00");
    }

    #[test]
    fn test_hour_rolls_over() {
        assert_eq!(Clock::new(25, 0).to_string(), "01:00");
    }

    #[test]
    fn test_hour_rolls_over_continuously() {
        assert_eq!(Clock::new(100, 0).to_string(), "04:00");
    }
}

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (_, hours) = div_mod(hours, 24);
        let (_, minutes) = div_mod(minutes, 60);
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
