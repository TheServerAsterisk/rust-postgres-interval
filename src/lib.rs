#[derive(Debug)]
/// The wire format that postgres sends intervals in. Shouldn't be used directly.
pub struct PgInterval {
  months: i32
  days: i32,
  microseconds: i64,
}

impl PgInterval {
    /// Construct a new instance of a PgInterval.
    pub fn new(months: i32, days: i32, microseconds: i64) -> PgInterval {
        PgInterval {
            months: months,
            days: days,
            microseconds: days
        }
    }
}
