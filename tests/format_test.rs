extern crate time_interval;
use self::time_interval::Interval;

#[test]
fn full_sql_format() {
    let interval = Interval::new(1,1,3,1,23,23.001);
    let format = interval.to_sql_standard();
    assert!("+1-1 +3 1:23:23.001" == &*format)
}

#[test]
fn full_sql_format_neg_year_month() {
    let interval = Interval::new(-1,-1,3,1,23,23.001);
    let format = interval.to_sql_standard();
    assert!("-1-1 +3 1:23:23.001" == &*format)
}

#[test]
fn full_sql_format_neg_day() {
    let interval = Interval::new(-1,-1,-3,1,23,23.001);
    let format = interval.to_sql_standard();
    assert!("-1-1 -3 1:23:23.001" == &*format)
}

#[test]
fn full_sql_format_neg_time() {
    let interval = Interval::new(-1,-1,-3,-1,-23,-23.001);
    let format = interval.to_sql_standard();
    assert!("-1-1 -3 -1:23:23.001" == &*format)
}

#[test]
fn year_month_only_sql_format() {
    let interval = Interval::new(1,1,0,0,0,0.0);
    let format = interval.to_sql_standard();
    assert!("1-1" == &*format)
}

#[test]
fn neg_year_month_only_sql_format() {
    let interval = Interval::new(-1,-1,0,0,0,0.0);
    let format = interval.to_sql_standard();
    assert!("-1-1" == &*format)
}

#[test]
fn day_only_sql_format() {
    let interval = Interval::new(0,0,1,0,0,0.0);
    let format = interval.to_sql_standard();
    assert!("1 0:00:00" == &*format)
}

#[test]
fn neg_day_only_sql_format() {
    let interval = Interval::new(0,0,-1,0,0,0.0);
    let format = interval.to_sql_standard();
    assert!("-1 0:00:00" == &*format)
}
