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

    #[test]
    fn test_hours_and_minutes_roll_over_to_exactly_midnight() {
        assert_eq!(Clock::new(72, 86400).to_string(), "00:00");
    }

    #[test]
    fn test_negative_hour() {
        assert_eq!(Clock::new(-1, 15).to_string(), "23:15");
    }

    #[test]
    fn test_negative_hour_roll_over_continuously() {
        assert_eq!(Clock::new(-91, 0).to_string(), "05:00")
    }

    #[test]
    fn test_negative_minutes() {
        assert_eq!(Clock::new(1, -40).to_string(), "00:20");
    }

    #[test]
    fn test_negative_minutes_roll_over() {
        assert_eq!(Clock::new(1, -160).to_string(), "22:20");
    }

    #[test]
    fn test_negative_minutest_roll_over_continuously() {
        assert_eq!(Clock::new(1, -4820).to_string(), "16:40")
    }

    #[test]
    fn test_negative_sixty_minutes_is_prev_hour() {
        assert_eq!(Clock::new(2, -60).to_string(), "01:00");
    }

    #[test]
    fn test_negative_one_twenty_minutes_is_two_prev_hours() {
        assert_eq!(Clock::new(1, -120).to_string(), "23:00");
    }

    #[test]
    fn test_negative_hour_and_minutes_both_roll_over() {
        assert_eq!(Clock::new(-25, -160).to_string(), "20:20");
    }

    #[test]
    fn test_negative_hour_and_minutes_both_roll_over_continuously() {
        assert_eq!(Clock::new(-121, -5810).to_string(), "22:10");
    }

    #[test]
    fn test_zero_hour_and_negative_minutes() {
        assert_eq!(Clock::new(0, -22).to_string(), "23:38");
    }

    #[test]
    fn test_add_minutes() {
        let clock = Clock::new(10, 0).add_minutes(3);
        assert_eq!(clock.to_string(), "10:03");
    }

    #[test]
    fn test_add_no_minutes() {
        let clock = Clock::new(6, 41).add_minutes(0);
        assert_eq!(clock.to_string(), "06:41");
    }

    #[test]
    fn test_add_to_next_hour() {
        let clock = Clock::new(0, 45).add_minutes(40);
        assert_eq!(clock.to_string(), "01:25");
    }

    #[test]
    fn test_add_more_than_one_hour() {
        let clock = Clock::new(10, 0).add_minutes(61);
        assert_eq!(clock.to_string(), "11:01");
    }

    #[test]
    fn test_add_more_than_two_hours_with_carry() {
        let clock = Clock::new(0, 45).add_minutes(160);
        assert_eq!(clock.to_string(), "03:25");
    }

    #[test]
    fn test_add_across_midnight() {
        let clock = Clock::new(23, 59).add_minutes(2);
        assert_eq!(clock.to_string(), "00:01");
    }

    #[test]
    fn test_add_more_than_one_day() {
        let clock = Clock::new(5, 32).add_minutes(1500);
        assert_eq!(clock.to_string(), "06:32");
    }

    #[test]
    fn test_add_more_than_two_day() {
        let clock = Clock::new(1, 1).add_minutes(3500);
        assert_eq!(clock.to_string(), "11:21");
    }

    #[test]
    fn test_subtract_minutes() {
        let clock = Clock::new(10, 3).add_minutes(-3);
        assert_eq!(clock.to_string(), "10:00");
    }

    #[test]
    fn test_subtract_to_previous_hour() {
        let clock = Clock::new(10, 3).add_minutes(-30);
        assert_eq!(clock.to_string(), "09:33");
    }

    #[test]
    fn test_subtract_more_than_an_hour() {
        let clock = Clock::new(10, 3).add_minutes(-70);
        assert_eq!(clock.to_string(), "08:53");
    }
}

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hour, minutes) = div_mod(minutes, 60);
        let hours = hours % 24;
        let hour = if minutes < 0 {
            (hour - 1) % 24
        } else {
            hour % 24
        };
        let hours = (hours + hour + 24) % 24;
        let minutes = (minutes + 60) % 60;
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
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
