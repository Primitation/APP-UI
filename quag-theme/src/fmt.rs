//! Number formatting shared by all parts, so units read the same everywhere (DESIGN.md "Numbers, units").
//! The thresholds and units are in `consts::format`.

use crate::consts::format::{BYTE_UNITS as U, BYTES_STEP, DAY, HOUR, MINUTE, NOW_SECS, ONE_DECIMAL_BELOW};

/// Bytes in binary units: `512 B`, `8.5 KiB`, `412 MiB`, `1.4 GiB`.
pub fn bytes(b: u64) -> String {
    let mut v = b as f64;
    let mut i = 0;
    while v >= BYTES_STEP && i < U.len() - 1 { v /= BYTES_STEP; i += 1; }
    if i == 0 { format!("{b} B") } else if v < ONE_DECIMAL_BELOW { format!("{} {}", trim(format!("{v:.1}")), U[i]) } else { format!("{v:.0} {}", U[i]) }
}

/// `used / limit` with one unit: `412 / 512 MiB` (the unit of the limit).
pub fn bytes_of(used: u64, limit: u64) -> String {
    let l = bytes(limit);
    let (num, unit) = l.split_once(' ').unwrap_or((&l, ""));
    let div = BYTES_STEP.powi(U.iter().position(|u| *u == unit).unwrap_or(0) as i32);
    let u = used as f64 / div;
    let u = if u < ONE_DECIMAL_BELOW && div > 1.0 { trim(format!("{u:.1}")) } else { format!("{u:.0}") };
    format!("{u} / {num} {unit}")
}

/// How long ago: `now`, `12s ago`, `3m ago`, `2h ago`, `4d ago`.
pub fn age(secs: u64) -> String {
    match secs {
        s if s < NOW_SECS => "now".into(),
        s if s < MINUTE => format!("{s}s ago"),
        s if s < HOUR => format!("{}m ago", s / MINUTE),
        s if s < DAY => format!("{}h ago", s / HOUR),
        s => format!("{}d ago", s / DAY),
    }
}

/// How long something has run: `45s`, `12m`, `3h 20m`, `2d 4h`.
pub fn uptime(secs: u64) -> String {
    match secs {
        s if s < MINUTE => format!("{s}s"),
        s if s < HOUR => format!("{}m", s / MINUTE),
        s if s < DAY => format!("{}h {}m", s / HOUR, s % HOUR / MINUTE),
        s => format!("{}d {}h", s / DAY, s % DAY / HOUR),
    }
}

fn trim(s: String) -> String {
    if s.contains('.') { s.trim_end_matches('0').trim_end_matches('.').to_string() } else { s }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn units() {
        assert_eq!(bytes(512), "512 B");
        assert_eq!(bytes(412 * 1024 * 1024), "412 MiB");
        assert_eq!(bytes(1_503_238_553), "1.4 GiB");
        assert_eq!(bytes(1024 * 1024 * 1024), "1 GiB");
        assert_eq!(bytes_of(412 * 1024 * 1024, 512 * 1024 * 1024), "412 / 512 MiB");
        assert_eq!(bytes_of(900 * 1024 * 1024, 2 * 1024 * 1024 * 1024), "0.9 / 2 GiB");
        assert_eq!(age(3), "now");
        assert_eq!(age(190), "3m ago");
        assert_eq!(uptime(12_000), "3h 20m");
        assert_eq!(uptime(187_200), "2d 4h");
    }
}
