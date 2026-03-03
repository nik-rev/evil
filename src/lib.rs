#![doc = concat!("[![crates.io](https://img.shields.io/crates/v/", env!("CARGO_PKG_NAME"), "?style=flat-square&logo=rust)](https://crates.io/crates/", env!("CARGO_PKG_NAME"), ")")]
#![doc = concat!("[![docs.rs](https://img.shields.io/docsrs/", env!("CARGO_PKG_NAME"), "?style=flat-square&logo=docs.rs)](https://docs.rs/", env!("CARGO_PKG_NAME"), ")")]
#![doc = "![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)"]
//! ![msrv](https://img.shields.io/badge/msrv-nightly-blue?style=flat-square&logo=rust)
//! [![github](https://img.shields.io/github/stars/nik-rev/auto-default)](https://github.com/nik-rev/auto-default)
//!
//! This crate lets you use the `?` operator as a shorthand for `.unwrap()`. Works on both. Works on both [`Result`](core::result::Result) and [`Option`](core::option::Option)
//!
//! ```toml
#![doc = concat!(env!("CARGO_PKG_NAME"), " = ", "\"", env!("CARGO_PKG_VERSION_MAJOR"), ".", env!("CARGO_PKG_VERSION_MINOR"), "\"")]
//! ```
//!
//! # But... why?
//!
//! As much as we'd like to not admit it, there *are* situations where calling `.unwrap()` on everything is okay, preferred by many over traditional error handling. Almost exclusively, this will be **inside of tests** and **small scripts**.
//!
//! In these cases, error handling ergonomics are extremely important - and we don't want them to be in the way. We
//!
//! You just want to pretend you're always on the happy path. That's what tests are usually for.
//!
//! This is what this crate, `evil`, is all about. `evil`
//!
//! Let's be honest. We're *tired* of writing `.unwrap()` all the time.
//!
//! > "**All the time**???, but what about the error handling!"
//!
//! There are 2 cases where writing `.unwrap()` is *acceptable* over proper error handling:
//!
//! - **Quick and dirty scripts**. I'm talking less than *100* lines of code!
//!
//!   The fact that `.unwrap()` tells you the exact location is *amazing*. That's what makes it better than a return type of `Result<(), Box<dyn std::error::Error>`.
//!
//!   If you use `Result<(), Box<dyn std::error::Error>` as a return type from `main`, using `?` operator **won't** tell you where the error happened!
//!
//!   Another alternative is having the return type of your function be [`eyre::Result<()>`](https://docs.rs/eyre/latest/eyre/type.Result.html). This is, like, **way** better.
//!
//!   But it's still not ergonomic ideal. If you're only using [`eyre::Result<()>`] as a return type because the syntax is lighter compared to all of those `.unwrap()`s, then
//!   we still have a long way to go. Yes, you can use the `?` operator on any `Result` type that you come across, **from anywhere**.
//!
//!   But [`eyre::Result<()>`] has one *huge* drawback: The `?` operator doesn't work on `Option`s. You are **forced** to import the [`OptionExt`](https://docs.rs/eyre/latest/eyre/trait.OptionExt.html) trait and call [Context::ok_or_eyre](https://docs.rs/eyre/latest/eyre/trait.OptionExt.html#tymethod.ok_or_eyre) method on every single `Option` that you want to pretend is always `Some`:

pub struct Evil;
