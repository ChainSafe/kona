## `kona-genesis`

<a href="https://github.com/op-rs/kona/actions/workflows/rust_ci.yaml"><img src="https://github.com/op-rs/kona/actions/workflows/rust_ci.yaml/badge.svg?label=ci" alt="CI"></a>
<a href="https://crates.io/crates/kona-genesis"><img src="https://img.shields.io/crates/v/kona-genesis.svg" alt="kona-genesis crate"></a>
<a href="https://github.com/op-rs/kona/blob/main/LICENSE.md"><img src="https://img.shields.io/badge/License-MIT-d1d1f6.svg?label=license&labelColor=2a2f35" alt="MIT License"></a>
<a href="https://op-rs.github.io/kona"><img src="https://img.shields.io/badge/Book-854a15?logo=mdBook&labelColor=2a2f35" alt="Book"></a>


Genesis types for Optimism.

### Usage

_By default, `kona-genesis` enables both `std` and `serde` features._

If you're working in a `no_std` environment (like [`kona`][kona]), disable default features like so.

```toml
[dependencies]
kona-genesis = { version = "x.y.z", default-features = false, features = ["serde"] }
```

#### Rollup Config

`kona-genesis` exports a `RollupConfig`, the primary genesis type for Optimism Consensus.


<!-- Links -->

[alloy-genesis]: https://github.com/alloy-rs
[kona]: https://github.com/op-rs/kona/blob/main/Cargo.toml#L137
