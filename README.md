# Human bytes
[![License](https://img.shields.io/crates/l/human_bytes-rs?style=flat-square)](https://gitlab.com/forkbomb9/human_bytes-rs/-/blob/master/LICENSE)
[![Latest version](https://img.shields.io/crates/v/human_bytes?style=flat-square)](https://crates.io/crates/human_bytes)
[![Build status](https://img.shields.io/gitlab/pipeline/forkbomb9/human_bytes-rs?style=flat-square)]()

> `human_bytes` is a Rust crate to convert bytes into human-readable values.

# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
human_bytes = "0.1"
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

# About
This crate is based on a PHP function I found [here](https://math.stackexchange.com/questions/247444/explain-convertion-algorithm-from-bytes-to-kb-mb-gb).

It is useful because you don't have to provide a prefix, it does it on its own.
This means it'll return the correct prefix, and never return "1000 KB", always "1 MB"

It has some tests I wrote to check that the conversion is correct, and it returns decimals, e.g. "16.5 GB"

# Limitations
The function uses `f64` floating point numbers,
which ranges from $`±0000000000000000×10^{−398}`$ to $`±9999999999999999×10^{369}`$ acording to [Wikipedia](https://en.wikipedia.org/wiki/Decimal64_floating-point_format)
That's a number so big, that running `human_bytes(std::f64::MAX)` `panic!`'s with `'index out of bounds: the len is 9 but the index is 102'`.
That's because I only have suffixes up to [yottabyte](https://en.wikipedia.org/wiki/Yottabyte)s, which quoting Wikipedia

> In 2010, it was estimated that storing a yottabyte on terabyte-size disk drives would require one million city block-size data-centers, as big as the states of Delaware and Rhode Island combined.[1] By late 2016 memory density had increased to the point where a yottabyte could be stored on SD cards occupying roughly twice the size of the Hindenburg[2] (around 400 thousand cubic metres).
so I decided not to go bigger (and anyway, the [SI](https://en.wikipedia.org/wiki/International_System_of_Units) doesn't define bigger prefixes).

I use `f64` to be able of returning decimals (e.g. `5.3 GB`), but if you want to use `u64` bytes, you have to write `10_u64 as f64`, because `f64` dousn't implement `From<u64>`.
And no way you're going to use `u128`'s. Anyway, `human_bytes(std::u64::MAX as f64)` returns `16 EB`, which is more than Google has on their servers ([source](https://en.wikipedia.org/wiki/Exabyte#Google)).

For now, I only test correct conversions up to 2.5 terabytes.

# License
BSD 2-clause - Copyright (c) 2020 Namkhai Bourquin (forkbomb9)
