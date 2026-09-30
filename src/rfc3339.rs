//! Minimal RFC 3339 support for the timestamps of the contract, so the SDK
//! needs no date and time dependency.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Formats a time as `2026-09-21T03:00:05Z`, dropping sub second precision.
pub(crate) fn format_utc(time: SystemTime) -> String {
    let secs = match time.duration_since(UNIX_EPOCH) {
        Ok(d) => i64::try_from(d.as_secs()).unwrap_or(i64::MAX),
        Err(e) => -i64::try_from(e.duration().as_secs()).unwrap_or(i64::MAX),
    };
    let days = secs.div_euclid(86_400);
    let rem = secs.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        rem / 3600,
        rem % 3600 / 60,
        rem % 60
    )
}

/// Parses an RFC 3339 timestamp with an optional fraction and a `Z` or
/// numeric offset. It returns `None` for anything else.
pub(crate) fn parse(text: &str) -> Option<SystemTime> {
    let bytes = text.as_bytes();
    if bytes.len() < 20 {
        return None;
    }
    let num = |range: std::ops::Range<usize>| -> Option<i64> {
        let part = text.get(range)?;
        if part.bytes().all(|b| b.is_ascii_digit()) {
            part.parse().ok()
        } else {
            None
        }
    };
    let separators_ok = bytes[4] == b'-'
        && bytes[7] == b'-'
        && (bytes[10] == b'T' || bytes[10] == b't' || bytes[10] == b' ')
        && bytes[13] == b':'
        && bytes[16] == b':';
    if !separators_ok {
        return None;
    }
    let (year, month, day) = (num(0..4)?, num(5..7)?, num(8..10)?);
    let (hour, minute, second) = (num(11..13)?, num(14..16)?, num(17..19)?);
    if !(1..=12).contains(&month)
        || !(1..=31).contains(&day)
        || hour > 23
        || minute > 59
        || second > 60
    {
        return None;
    }

    let mut idx = 19;
    let mut nanos: u32 = 0;
    if bytes[idx] == b'.' {
        idx += 1;
        let start = idx;
        while idx < bytes.len() && bytes[idx].is_ascii_digit() {
            idx += 1;
        }
        if idx == start {
            return None;
        }
        let digits = &text[start..idx.min(start + 9)];
        nanos = digits.parse::<u32>().ok()? * 10u32.pow(9 - u32::try_from(digits.len()).ok()?);
    }

    let offset_secs: i64 = match bytes.get(idx)? {
        b'Z' | b'z' if idx + 1 == bytes.len() => 0,
        sign @ (b'+' | b'-') if idx + 6 == bytes.len() && bytes[idx + 3] == b':' => {
            let oh = num(idx + 1..idx + 3)?;
            let om = num(idx + 4..idx + 6)?;
            let total = oh * 3600 + om * 60;
            if *sign == b'-' {
                -total
            } else {
                total
            }
        }
        _ => return None,
    };

    let days = days_from_civil(year, month, day);
    let secs = days * 86_400 + hour * 3600 + minute * 60 + second - offset_secs;
    let base = if secs >= 0 {
        UNIX_EPOCH.checked_add(Duration::from_secs(u64::try_from(secs).ok()?))?
    } else {
        UNIX_EPOCH.checked_sub(Duration::from_secs(u64::try_from(-secs).ok()?))?
    };
    base.checked_add(Duration::from_nanos(u64::from(nanos)))
}

// Both conversions follow Howard Hinnant's civil calendar algorithms.

fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(secs: u64) -> SystemTime {
        UNIX_EPOCH + Duration::from_secs(secs)
    }

    #[test]
    fn formats_and_parses_round_trip() {
        for secs in [0, 86_399, 951_782_400, 1_790_000_000, 4_102_444_800] {
            let text = format_utc(at(secs));
            assert_eq!(parse(&text), Some(at(secs)), "{text}");
        }
        assert_eq!(format_utc(at(1_790_000_000)), "2026-09-21T14:13:20Z");
        assert_eq!(format_utc(at(1_789_959_605)), "2026-09-21T03:00:05Z");
    }

    #[test]
    fn parses_fractions_and_offsets() {
        assert_eq!(parse("2026-09-23T08:15:02Z"), Some(at(1_790_151_302)));
        let base = parse("2026-09-23T08:15:02Z").unwrap();
        assert_eq!(
            parse("2026-09-23T08:15:02.5Z"),
            Some(base + Duration::from_millis(500))
        );
        assert_eq!(
            parse("2026-09-23T10:15:02+02:00"),
            Some(base),
            "a positive offset is ahead of UTC"
        );
        assert_eq!(parse("2026-09-23T06:45:02-01:30"), Some(base));
        assert_eq!(parse("2026-09-23"), None);
        assert_eq!(parse("not a timestamp at all"), None);
        assert_eq!(parse("2026-13-23T08:15:02Z"), None);
    }
}
