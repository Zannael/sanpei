/// Require an actual UTC instant instead of accepting arbitrary strings that
/// happen to sort like a timestamp. Intervals use one canonical precision.
pub fn valid_utc(value: &str, canonical_seconds: bool) -> bool {
    if !value.ends_with('Z') || (canonical_seconds && value.len() != 20) {
        return false;
    }
    chrono::DateTime::parse_from_rfc3339(value)
        .is_ok_and(|time| time.offset().local_minus_utc() == 0)
}

#[cfg(test)]
mod tests {
    use super::valid_utc;
    #[test]
    fn validates_real_utc_dates_and_interval_precision() {
        assert!(valid_utc("2026-10-25T01:30:00Z", true));
        assert!(!valid_utc("2026-02-30T01:30:00Z", true));
        assert!(!valid_utc("2026-10-25T01:30:00+02:00", false));
        assert!(!valid_utc("2026-10-25T01:30:00.000Z", true));
        assert!(valid_utc("2026-10-25T01:30:00.000Z", false));
    }
}
