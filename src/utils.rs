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

pub fn conv_t(dur: std::time::Duration) -> String {
    let d = chrono::Duration::from_std(dur).unwrap();
    format!("{} days, {} hours {} minutes {} seconds.", d.num_days(), d.num_hours(), d.num_minutes(), d.num_seconds() )
}