use std::fmt;


#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut new_hours = (minutes / 60 + hours).rem_euclid(24);
        let mut new_minutes = minutes.rem_euclid(60);

        if minutes.is_negative() && new_minutes != 0 {
            new_hours = (new_hours - 1).rem_euclid(24);
        }

        Clock {
            hours: new_hours,
            minutes: new_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut new_hours = (minutes / 60 + self.hours).rem_euclid(24);
        let mut new_minutes = minutes.rem_euclid(60) + self.minutes;

        if new_minutes >= 60 {
            new_hours = (new_hours + 1).rem_euclid(24);
            new_minutes = new_minutes.rem_euclid(60);
        }

        if minutes.is_negative() && minutes.rem_euclid(60) != 0 {
            new_hours = (new_hours - 1).rem_euclid(24);
        }

        Clock {
            hours: new_hours,
            minutes: new_minutes,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

// impl From<Clock> for String {
//     fn from(clock: Clock) -> Self {
//         format!("{:0>2}:{:0>2}", clock.hours, clock.minutes)
//     }
// }

// // impl Into<String> for Clock {
// //     fn into(self) -> String {
// //         format!("{:0>2}:{:0>2}", self.hours, self.minutes)
// //     }
// // }
//
// impl ToString for Clock {
//     fn to_string(&self) -> String {
//         format!("{:0>2}:{:0>2}", self.hours, self.minutes)
//     }
// }
