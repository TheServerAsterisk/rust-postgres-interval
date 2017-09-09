const MONTHS_TO_YEAR: i32 = 12;
const MICROS_TO_HOUR: i64 = 3600000000;
const MICROS_TO_MINUTE: i64 = 60000000;
const MICROS_TO_SECOND: i64 = 1000000;

pub fn get_year_month_interval(months: i32) -> (i32, i8) {
    let years: i32 = months/MONTHS_TO_YEAR;
    let months: i8 = (months - (years * MONTHS_TO_YEAR)) as i8;
    (years, months)
}

pub fn get_day_time_interval(microseconds: i64) -> (i64, i8, f64) {
    let hours: i64 = microseconds/MICROS_TO_HOUR;
    let current_hours: i64 = hours * MICROS_TO_HOUR;
    let minutes: i64 = (microseconds - current_hours)/MICROS_TO_MINUTE;
    let seconds: f64 = (microseconds - (current_hours+(minutes * MICROS_TO_MINUTE))) as f64/MICROS_TO_SECOND as f64;
    (hours, minutes as i8, seconds)
}
