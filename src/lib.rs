mod helpers;

use helpers::*;

/// An Iso-8601 Time Interval Representation.
pub struct Interval {
    years: i32,
    months: i8,
    days: i32,
    hours: i64,
    minutes: i8,
    seconds: f64
}

impl Interval {
    pub fn new(years: i32,
               months: i8,
               days: i32,
               hours: i64,
               minutes: i8,
               seconds: f64) -> Interval {
        Interval {
            years: years,
            months: months,
            days: days,
            hours: hours,
            minutes: minutes,
            seconds: seconds
        }
    }


    /// Get the amount of years that the interval spans.
    pub fn get_years(&self) -> i32 {
        self.years
    }

    /// Get the amount of months that the interval spans.
    pub fn get_months(&self) -> i8 {
        self.months
    }

    /// Get the amount of days that the interval spans.
    pub fn get_days(&self) -> i32 {
        self.days
    }

    /// Get the amount of hourss that the interval spans.
    pub fn get_hours(&self) -> i64 {
        self.hours
    }

    /// Get the amount of minutes that the interval spans.
    pub fn get_minutes(&self) -> i8 {
        self.minutes
    }

    /// Get the amount of seconds that the interval spans. Anything smaller
    /// than a seconds is represented fractionally.
    pub fn get_seconds(&self) -> f64 {
        self.seconds
    }

    /// Map interval formats that use months, days, and microseconds to Interval.
    pub fn from_months_days_microseconds(months: i32,
                                        days: i32,
                                        microseconds: i64)
                                         -> Interval {
        let (years,months) = get_year_month_interval(months);
        let (hours, minutes, seconds) = get_day_time_interval(microseconds);
        Interval {
            years: years,
            months: months,
            days: days,
            hours: hours,
            minutes: minutes,
            seconds: seconds
        }
    }

    pub fn to_sql_standard(&self) -> String {
        let only_nothing = self.is_year_month_zero() &&
                           self.is_time_zero() &&
                           self.is_day_zero();
        let only_year_month = !self.is_year_month_zero() &&
                              self.is_time_zero() &&
                              self.is_day_zero();
        let only_day = !self.is_day_zero() &&
                       self.is_time_zero() &&
                       self.is_year_month_zero();
        let only_time = !self.is_time_zero() &&
                        self.is_year_month_zero() &&
                        self.is_day_zero();
        if only_nothing {
            "0".to_owned()
        } else if only_year_month {
            let months_abs = get_absolute(self.months);
            format!("{}-{}", self.years, months_abs)
        } else if only_day {
          format!("{} 0:00:00", self.days)
        } else if only_time {
            let min_abs = get_absolute(self.minutes);
            let (whole_seconds, remainder) = split_seconds(self.seconds);
            if &*remainder == "0" {
                format!("{}:{:02}:{:02}",
                        self.hours,
                        min_abs,
                        whole_seconds)
            } else {
                format!("{}:{:02}:{:02}.{}",
                        self.hours,
                        min_abs,
                        whole_seconds,
                        remainder)
            }
        } else {
            let months_abs = get_absolute(self.months);
            let min_abs = get_absolute(self.minutes);
            let (whole_seconds, remainder) = split_seconds(self.seconds);
            if &*remainder == "0" {
                format!("{:+}-{} {:+} {}:{:02}:{:02}",
                        self.years,
                        months_abs,
                        self.days,
                        self.hours,
                        min_abs,
                        whole_seconds)
            } else {
                format!("{:+}-{} {:+} {}:{:02}:{:02}.{}",
                        self.years,
                        months_abs,
                        self.days,
                        self.hours,
                        min_abs,
                        whole_seconds,
                        remainder)
            }
        }

    }

    fn is_year_month_zero(&self) -> bool {
        self.years == 0 || self.months == 0
    }

    fn is_time_zero(&self) -> bool {
        self.hours == 0 && self.minutes == 0 && self.seconds == 0.0
    }

    fn is_day_zero(&self) -> bool {
        self.days == 0
    }
}
