mod helpers;

use helpers::*;

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
}
