pub fn convert_seconds(seconds: i32) -> (u64, u64, u64, u64) {
    let days = seconds / (24 * 3600);
    let mut remain_seconds = seconds % (24 * 3600);

    let hours = remain_seconds / 3600;
    remain_seconds %= 3600;

    let minutes = remain_seconds / 60;
    remain_seconds %= 60;

    (days as u64, hours as u64, minutes as u64, remain_seconds as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seconds_partition_low_boundary() {
        let expected = (0, 0, 0, 1);
        let (days, hours, minutes, seconds) = convert_seconds(1);
        assert_eq!((days, hours, minutes, seconds), expected);
    }

    #[test]
    fn test_seconds_partition_high_boundary() {
        let expected = (0, 0, 0, 59);
        let (days, hours, minutes, seconds) = convert_seconds(59);
        assert_eq!((days, hours, minutes, seconds), expected);
    }

    #[test]
    fn test_minutes_partition_low_boundary() {
        let expected = (0, 0, 1, 0);
        let (days, hours, minutes, seconds) = convert_seconds(1 * 60);
        assert_eq!((days, hours, minutes, seconds), expected);
    }

    #[test]
    fn test_minutes_partition_high_boundary() {
        let expected = (0, 0, 59, 59);
        let (days, hours, minutes, seconds) = convert_seconds(59 * 60 + 59);
        assert_eq!((days, hours, minutes, seconds), expected);
    }

    #[test]
    fn test_hours_partition_low_boundary() {
        let expected = (0, 1, 0, 0);
        let (days, hours, minutes, seconds) = convert_seconds(1 * 60 * 60);
        assert_eq!((days, hours, minutes, seconds), expected);
    }

    #[test]
    fn test_hours_partition_high_boundary() {
        let expected = (0, 23, 59, 59);
        let (days, hours, minutes, seconds) = convert_seconds(23 * 60 * 60 + 59 * 60 + 59);
        assert_eq!((days, hours, minutes, seconds), expected);
    }

    #[test]
    fn test_days_partition_low_boundary() {
        let expected = (1, 0, 0, 0);
        let (days, hours, minutes, seconds) = convert_seconds(24 * 60 * 60);
        assert_eq!((days, hours, minutes, seconds), expected);
    }
}
