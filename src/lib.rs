//! # human_bytes
//!
//! `human_bytes` is a Rust crate to convert bytes into human-readable values.

//! # Example
//!
//! ```
//! use human_bytes::human_bytes;
//!
//! assert_eq!(human_bytes(563_200_u32), "550 KB".to_string());
//! // or
//! assert_eq!(human_bytes(563_200_u64 as f64), "550 KB".to_string());
//! // ________________________________/
//! // |
//! // | Needed only when you're using `u64` values,
//! // | because `f64` doesn't implement `std::convert::From<u64>`
//! ```
//! For more info, check the [README.md](https://gitlab.com/forkbomb9/human_bytes-rs)


/// Performs the conversion
pub fn human_bytes<T: Into<f64>>(size: T) -> String {
    let size = size.into();

    if size <= 0.0 {
        return "0 B".to_string();
    }

    let base = size.log10() / 1024_f64.log10();
    // Just be future-proof
    let suffix = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

    let stringed = format!("{:.1}", 1024_f64.powf(base - base.floor()));
    format!(
        "{} {}",
        stringed.trim_end_matches(".0"),
        suffix[base.floor() as usize]
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn nothing() {
        assert_eq!(super::human_bytes(0_u32), "0 B".to_string());
    }

    #[test]
    fn bytes() {
        assert_eq!(super::human_bytes(550_u32), "550 B".to_string());
    }
    #[test]
    fn kilobytes() {
        assert_eq!(super::human_bytes(563_200_u32), "550 KB".to_string());
    }

    #[test]
    fn megabytes() {
        assert_eq!(super::human_bytes(681_574_400_u32), "650 MB".to_string());
    }

    #[test]
    fn gigabytes() {
        assert_eq!(super::human_bytes(16_428_249_907_u64 as f64), "15.3 GB".to_string());
    }

    #[test]
    fn terabytes() {
        // Hacky, I know, but easier to write ;)
        let terabyte: u64 = 2_u64.pow(40);
        assert_eq!(super::human_bytes(((terabyte * 2) + (terabyte / 2)) as f64), "2.5 TB");
    }
}
