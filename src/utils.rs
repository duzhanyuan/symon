pub fn conv_p(total: u64, free: u64) -> u64 {
    if total != 0 { free * 100 / total } else { 0 }
}

pub fn conv_b(bytes: u64) -> String {
    let n: f64 = bytes as f64;
    let s: (f64, &str) = if n < 1024. { (n, "B") }
    else if 1024. <= n && n < u64::pow(1024, 2) as f64 {
        (n / 1024., "KB")
    }
    else if u64::pow(1024, 2) as f64 <= n && n < u64::pow(1024, 3) as f64 {
        (n / u64::pow(1024, 2) as f64, "MB")
    }
    else if u64::pow(1024, 3) as f64 <= n && n < u64::pow(1024, 4) as f64 {
        (n / u64::pow(1024, 3) as f64, "GB")
    }
    else {
        (n / u64::pow(1024, 4) as f64, "TB")
    };
    format!("{:.2} {}", s.0, s.1)
}

pub fn conv_t(sec: f64) -> String {
    if sec < 60. { format!("{} seconds", sec) }
    else if 60. <= sec && sec < u64::pow(60, 2) as f64{
        let minutes = (sec / 60.).floor();
        let seconds = (sec % 60.).floor();
        format!("{} minutes {} seconds", minutes, seconds)
    }
    else if u64::pow(60, 2) as f64 <= sec && sec < u64::pow(60, 3) as f64{
        let hours = (sec / u64::pow(60, 2) as f64).floor();
        let minutes = ((sec % u64::pow(60, 2) as f64) / 60.).floor();
        let seconds = ((sec % u64::pow(60, 2) as f64) % 60.).floor();
        format!("{} hours {} minutes {} seconds", hours, minutes, seconds)
    }
    else {
        let days = (sec / (u64::pow(60, 2) as f64 * 24.)).floor();
        let hours = ((sec % (u64::pow(60, 2) as f64 * 24.)) / u64::pow(60, 2) as f64).floor();
        let minutes = (((sec % (u64::pow(60, 2) as f64 * 24.)) % u64::pow(60, 2) as f64) / 60.).floor();
        let seconds = (((sec % (u64::pow(60, 2) as f64 * 24.)) % u64::pow(60, 2) as f64) % 60.).floor();
        format!("{} days {} hours {} minutes {} seconds", days, hours, minutes, seconds)
    }
}