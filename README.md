[![Dependency status](https://deps.rs/repo/github/alexkazik/exomizer/status.svg)](https://deps.rs/repo/github/alexkazik/exomizer)
[![crates.io](https://img.shields.io/crates/v/exomizer.svg)](https://crates.io/crates/exomizer)
[![Downloads](https://img.shields.io/crates/d/exomizer.svg)](https://crates.io/crates/exomizer)
[![Github stars](https://img.shields.io/github/stars/alexkazik/exomizer.svg?logo=github)](https://github.com/alexkazik/exomizer/stargazers)
[![License](https://img.shields.io/crates/l/exomizer.svg)](./LICENSE)

# crate exomizer

<!-- cargo-rdme start -->

Exomizer is a program that compresses files in a way that tries to be as efficient as possible
but still allows them to be decompressed in environments where CPU speed and RAM are limited.

While the [original project](https://bitbucket.org/magli143/exomizer/wiki/Home) has a binary
which not only allows de-/crunching it is also capable to create self-extracting programs for
some popular 8-bit computers.

This library only supports de-/crunching in "raw" mode. There are two modules:
- original: Routines for de-/crunching, converted from the original C code.
  Requires `alloc`.
- simple: Routines for decrunching only, also only a subset of all possible
  parameters are supported (is `no_std`).

## Features

For original:
- `alloc` (default): enables the original routines.
- `std`: provides logging to io (incl. stdio) (you can write your own without it).

For simple:
- `clz` (default): enable the use of clz ([`usize::leading_zeros`](core::primitive::usize::leading_zeros)),
  please diable if using simple and your cpu does not support it
  (the function is always available but emulated on cpus which doesn't have it).

Unless `std` is activates the library is `no_std`.

### Usage

With defaults (`alloc` and `clz`):
```toml
[dependencies]
lzss = "0.5"
```

Without `alloc`:
```toml
[dependencies]
lzss = { version = "0.5", default-features = false, features = ["clz"] }
```

<!-- cargo-rdme end -->
