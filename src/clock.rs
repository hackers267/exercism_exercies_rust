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

    #[test]
    fn test_sixty_minutes_is_next_hour() {
        assert_eq!(Clock::new(1, 60).to_string(), "02:00");
    }

    #[test]
    fn test_minutes_roll_over() {
        assert_eq!(Clock::new(0, 160).to_string(), "02:40");
    }

    #[test]
    fn test_minutes_roll_over_continuously() {
        assert_eq!(Clock::new(0, 1723).to_string(), "04:43");
    }

    #[test]
    fn test_hours_and_minutes_roll_over() {
        assert_eq!(Clock::new(25, 160).to_string(), "03:40");
    }

    #[test]
    fn test_hours_and_minutes_roll_over_continuously() {
        assert_eq!(Clock::new(201, 3001).to_string(), "11:01");
    }
}

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hour, minutes) = div_mod(minutes, 60);
        let (_, hours) = div_mod(hours + hour, 24);
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
