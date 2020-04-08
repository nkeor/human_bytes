# Human bytes

[![License](https://img.shields.io/crates/l/human_bytes?style=flat-square)](https://gitlab.com/forkbomb9/human_bytes-rs/-/blob/master/LICENSE)
[![Latest version](https://img.shields.io/crates/v/human_bytes?style=flat-square)](https://crates.io/crates/human_bytes)
[![Build status](https://img.shields.io/gitlab/pipeline/forkbomb9/human_bytes-rs?style=flat-square)]()

`human_bytes` is a Rust crate that converts bytes into human-readable values (KB, MB, etc).

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
human_bytes = "0.2"
```

And then

```rust
use human_bytes::human_bytes;

assert_eq!(human_bytes(563_200_u32), "550 KB".to_string());
// or
assert_eq!(human_bytes(563_200_u64 as f64), "550 KB".to_string());
// ________________________________/
// |
// | Needed only when you're using `u64` values,
// | because `f64` doesn't implement `std::convert::From<u64>`
```

`human_bytes` is dependency-free, but if you want an +/- 15% speed improvement, I have a `fast` feature (which depends on [lexical](https://github.com/Alexhuszagh/rust-lexical))

```toml
[dependencies]
human_bytes = { version = "0.2", features = [ "fast" ] }
```

## About
This crate is based on a PHP function I found [here](https://math.stackexchange.com/questions/247444/explain-convertion-algorithm-from-bytes-to-kb-mb-gb).

It is useful because you don't have to provide a prefix, it does it on its own.
This means it'll return the correct prefix, and never return "1000 KB", always "1 MB"

It has some tests I wrote to check that the conversion is correct, and it returns decimals (e.g. `16.5 GB`)

## Changelog
Check the [CHANGELOG.md](./CHANGELOG.md)

## License
BSD 2-clause - Copyright (c) 2020 Namkhai B. (forkbomb9)
