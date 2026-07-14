pub fn format_yen(wan: f64) -> String {
    if wan >= 10000.0 {
        let v = wan / 10000.0;
        let s = format!("{:.1}", v);
        let s = if s.ends_with(".0") {
            s[..s.len() - 2].to_string()
        } else {
            s
        };
        format!("{}\u{5104}\u{5186}", s)
    } else {
        let n = wan.round() as i64;
        format!("{}\u{4E07}\u{5186}", format_with_commas(n))
    }
}

pub fn format_percent(num: f64) -> String {
    let pct = (num * 1000.0).round() / 10.0;
    if num >= 0.0 {
        format!("+{}%", pct)
    } else {
        format!("{}%", pct)
    }
}

fn format_with_commas(n: i64) -> String {
    let abs = n.abs();
    let s = abs.to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut result = String::new();
    for (i, &b) in bytes.iter().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            result.push(',');
        }
        result.push(b as char);
    }
    if n < 0 {
        format!("-{}", result)
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_yen_small() {
        assert_eq!(format_yen(1500.0), "1,500\u{4E07}\u{5186}");
    }

    #[test]
    fn test_format_yen_large() {
        assert_eq!(format_yen(20000.0), "2\u{5104}\u{5186}");
    }

    #[test]
    fn test_format_yen_decimal() {
        assert_eq!(format_yen(25000.0), "2.5\u{5104}\u{5186}");
    }

    #[test]
    fn test_format_percent_positive() {
        assert_eq!(format_percent(0.123), "+12.3%");
    }

    #[test]
    fn test_format_percent_negative() {
        assert_eq!(format_percent(-0.05), "-5%");
    }
}