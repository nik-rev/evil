# The `evil` crate

<!-- cargo-reedme: start -->

<!-- cargo-reedme: info-start

    Do not edit this region by hand
    ===============================

    This region was generated from Rust documentation comments by `cargo-reedme` using this command:

        cargo +nightly reedme

    for more info: https://github.com/nik-rev/cargo-reedme

cargo-reedme: info-end -->

[![crates.io](https://img.shields.io/crates/v/auto-default?style=flat-square&logo=rust)](https://crates.io/crates/auto-default)
[![docs.rs](https://img.shields.io/docsrs/auto-default?style=flat-square&logo=docs.rs)](https://docs.rs/auto-default)
![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)
![msrv](https://img.shields.io/badge/msrv-nightly-blue?style=flat-square&logo=rust)
[![github](https://img.shields.io/github/stars/nik-rev/auto-default)](https://github.com/nik-rev/auto-default)

This crate provides an attribute macro `#[auto_default]`, which adds a default field value of
`Default::default()` to fields that do not have one.

```toml
auto-default = "0.2"
```

Note: `auto-default` has *zero* dependencies. Not even `syn`! The compile times are very fast.

## Showcase

Rust’s [default field values](https://github.com/rust-lang/rust/issues/132162) allow
the shorthand `Struct { field, .. }` instead of the lengthy `Struct { field, ..Default::default() }`

For `..` instead of `..Default::default()` to work,
your `Struct` needs **all** fields to have a default value.

This often means `= Default::default()` boilerplate on every field, because it is
very common to want field defaults to be the value of their `Default` implementation

### Before

```rust
#[derive(Default)]
pub struct Layout {
    order: u32 = Default::default(),
    location: Point = Default::default(),
    size: Size = Default::default(),
    content_size: Size = Default::default(),
    scrollbar_size: Size = Default::default(),
    border: Rect = Default::default(),
    padding: Rect = Default::default(),
    margin: Rect = Default::default(),
}
```

### With `#[auto_default]`

```rust
#[auto_default]
#[derive(Default)]
pub struct Layout {
    order: u32,
    location: Point,
    size: Size,
    content_size: Size,
    scrollbar_size: Size,
    border: Rect,
    padding: Rect,
    margin: Rect,
}
```

You can apply the [`#[auto_default]`](https://docs.rs/auto-default/latest/auto_default/attr.auto_default.html) macro to `struct`s with named fields, and `enum`s.

If any field or variant has the `#[auto_default(skip)]` attribute, a default field value of `Default::default()`
will **not** be added

## Global Import

This will make `#[auto_default]` globally accessible in your entire crate, without needing to import it:

```rust
#[macro_use(auto_default)]
extern crate auto_default;
```

<!-- cargo-reedme: end -->
