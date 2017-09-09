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

pub fn split_seconds(seconds: f64) -> (i16, String) {
    let seconds = seconds.to_string();
    let second_stamp: Vec<&str> = seconds.split(".").collect();
    let mut whole_seconds: i16 = second_stamp[0].parse().unwrap();
    whole_seconds = whole_seconds.abs();
    if let Some(remainder) = second_stamp.get(1) {
        (whole_seconds, (*remainder).to_owned())
    } else {
        (whole_seconds, "0".to_owned())
    }
}

pub fn get_absolute(number: i8) -> i16 {
    let mut number = number as i16;
    number = number.abs();
    number
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn negative_year_month_interval() {
        let months = i32::min_value();
        let (years, months) = get_year_month_interval(months);
        assert!(years == -178956970);
        assert!(months == -8);
    }

    #[test]
    fn positive_year_month_interval() {
        let months = i32::max_value();
        let (years, months) = get_year_month_interval(months);
        assert!(years == 178956970);
        assert!(months == 7);
    }

    #[test]
    fn negative_day_time_interval() {
        let microseconds = -7199000000;
        let (hours, minutes, seconds) = get_day_time_interval(microseconds);
        assert!(hours == -1);
        assert!(minutes == -59);
        assert!(seconds == -59.0);
    }

    #[test]
    fn postive_day_time_interval() {
        let microseconds = 7199000000;
        let (hours, minutes, seconds) = get_day_time_interval(microseconds);
        assert!(hours == 1);
        assert!(minutes == 59);
        assert!(seconds == 59.0);
    }

    #[test]
    fn day_time_remaining_milliseconds() {
        let microseconds = 7199001000;
        let (hours, minutes, seconds) = get_day_time_interval(microseconds);
        assert!(hours == 1);
        assert!(minutes == 59);
        assert!(seconds == 59.001);
    }

    #[test]
    fn day_time_remaining_millisecond() {
        let microseconds = 7199000001;
        let (hours, minutes, seconds) = get_day_time_interval(microseconds);
        assert!(hours == 1);
        assert!(minutes == 59);
        assert!(seconds == 59.000001);
    }
}
