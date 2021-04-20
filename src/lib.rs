//! # human_bytes
//!
//! `human_bytes` is a Rust crate to convert bytes into human-readable values.

//! # Example
//!
//! ```no_run
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

#[cfg(test)]
mod tests;

#[cfg(not(feature = "bibytes"))]
// Just be future-proof
const SUFFIX: [&'static str; 9] = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

#[cfg(feature = "bibytes")]
// Just be future-proof
const SUFFIX: [&'static str; 9] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"];

/// Performs the conversion
pub fn human_bytes<T: Into<f64>>(size: T) -> String {
    let size = size.into();

    if size <= 0.0 {
        return "0 B".to_string();
    }

    let base = size.log10() / 1024_f64.log10();

    #[cfg(feature = "fast")]
    // Source for this hack: https://stackoverflow.com/a/28656825
    let mut result = lexical::to_string((1024_f64.powf(base - base.floor()) * 10.0).round() / 10.0)
        .trim_end_matches(".0")
        .to_owned();
    // This is faster, but leaves you with things like "2.500000000000002 TB" or 15.299999999813716 GB.
    // let result = lexical::to_string(1024_f64.powf(base - base.floor()));

    #[cfg(not(feature = "fast"))]
    let mut result = format!("{:.1}", 1024_f64.powf(base - base.floor()),)
        .trim_end_matches(".0")
        .to_owned();

    // Add suffix
    result.push(' ');
    result.push_str(SUFFIX[base.floor() as usize]);

    result
}
