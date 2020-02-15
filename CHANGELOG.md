# Changelog
Notes significant changes to human_bytes

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
* A `fast` feature, which improves performance by using [lexical](https://github.com/Alexhuszagh/rust-lexical) instead of `format!` to convert `f64`s to strings
