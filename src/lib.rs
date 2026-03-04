#![doc = concat!("[![crates.io](https://img.shields.io/crates/v/", env!("CARGO_PKG_NAME"), "?style=flat-square&logo=rust)](https://crates.io/crates/", env!("CARGO_PKG_NAME"), ")")]
#![doc = concat!("[![docs.rs](https://img.shields.io/docsrs/", env!("CARGO_PKG_NAME"), "?style=flat-square&logo=docs.rs)](https://docs.rs/", env!("CARGO_PKG_NAME"), ")")]
#![doc = "![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)"]
//! ![msrv](https://img.shields.io/badge/msrv-nightly-blue?style=flat-square&logo=rust)
//! [![github](https://img.shields.io/github/stars/nik-rev/evil)](https://github.com/nik-rev/evil)
//!
//! This crate lets you use the `?` operator as a shorthand for `.unwrap()`. Works on both [`Result`](core::result::Result) and [`Option`](core::option::Option).
//!
//! ```toml
#![doc = concat!(env!("CARGO_PKG_NAME"), " = ", "\"", env!("CARGO_PKG_VERSION_MAJOR"), ".", env!("CARGO_PKG_VERSION_MINOR"), "\"")]
//! ```
//!
//! # Example
//!
//! The `evil` crate *significantly* reduces boilerplate in tests. Error handling *in tests* dilutes the substance of your test.
//!
//! By removing all that boilerplate, you are now free to spend your energy and focus on what you are *actually* testing.
//!
//! ## Before
//!
//! ```
//! # /*
//! #[test]
//! fn user_theme_preference() {
//!     let response = make_api_call("/user/profile/settings").unwrap();
//!     let json: Value = serde_json::from_str(&response).unwrap();
//!
//!     let theme = json
//!         .get("data")
//!         .unwrap()
//!         .get("attributes")
//!         .unwrap()
//!         .get("preferences")
//!         .unwrap()
//!         .get("theme")
//!         .unwrap()
//!         .as_str()
//!         .unwrap();
//!
//!     assert_eq!(theme, "dark");
//! }
//! # */
//! ```
//!
//! ## After
//!
//! Return [`evil::Result<()>`](Result) directly from your test's function.
//!
//! ```
//! # /*
//! #[test]
//! fn user_theme_preference() -> evil::Result<()> {
//!     let response = make_api_call("/user/profile/settings")?;
//!     let json: Value = serde_json::from_str(&response)?;
//!
//!     let theme = json
//!         .get("data")?
//!         .get("attributes")?
//!         .get("preferences")?
//!         .get("theme")?
//!         .as_str()?;
//!
//!     assert_eq!(theme, "dark");
//!     evil::Ok(())
//! }
//! # */
//! ```
//!
//! Each one of those `?` is equivalent to a `.unwrap()`.
//!
//! # Use the `evil` crate in scripts
//!
//! When writing small Rust scripts that will only be used by developers, `.unwrap()`ping everything instead of proper error handling is common.
//!
//! **But there is one huge disadvantage with that approach.**
//!
//! Scripts turn into programs much more often than we'd like. Then, refactoring all of that `.unwrap()` boilerplate into good error handling is a significant undertaking.
//!
//! If you use [`evil::Result<()>`](Result) from the get-go, later refactoring your script to use something like [`anyhow::Result<()>`](https://docs.rs/anyhow/latest/anyhow/type.Result.html) is much simpler - you're
//! already using the `?` operator *everywhere* anyway. It's a piece of cake.
//!
//! # Why should I use [`evil::Result<()>`](Result) instead of [`eyre::Result<()>`](https://docs.rs/eyre/latest/eyre/type.Result.html)?
//!
//! The benefits of unwrapping everything is that you get the exact file, line and column information on where the unwrap failed. That's **amazing**. It helps debugging tremendously.
//!
//! When returning <code>Result\<(), Box\<dyn [core::error::Error]\>\></code> from your function, you don't get that. That information is simply *discarded*. Good luck figuring out where the error came from if you just use `?`. When returning [`anyhow::Result<()>`](https://docs.rs/anyhow/latest/anyhow/type.Result.html), it's the same problem.
//!
//! **But [`eyre::Result<()>`](https://docs.rs/eyre/latest/eyre/type.Result.html) is built different. It is *special*.**
//!
//! <br>
//!
//! `eyre::Result<()>` actually tells you the file, line and column information of where you use the `?` operator. But it has one *huge* downside compared to `evil::Result<()>`: **It only works on `Result`s, not `Option`s.**
//!
//! <br>
//!
//! Let's come back to our example and rewrite it with `eyre`:
//!
//! ```
//! # /*
//! use eyre::OptionExt as _;
//!
//! #[test]
//! fn user_theme_preference() -> eyre::Result<()> {
//!     let response = make_api_call("/user/profile/settings")?;
//!     let json: Value = serde_json::from_str(&response)?;
//!
//!     let theme = json
//!         .get("data")
//!         .ok_or_eyre("I have to give a reason why this is not `None`")?
//!         .get("attributes")
//!         .ok_or_eyre("and for this one as well...")?
//!         .get("preferences")
//!         .ok_or_eyre(".....I'm getting tired of this.....")?
//!         .get("theme")
//!         .ok_or_eyre("...............")
//!         .as_str()
//!         .ok_or_eyre(":/");
//!
//!     assert_eq!(theme, "dark");
//!     Ok(())
//! }
//! # */
//! ```
//!
//! This is **even more verbose** than just using `.unwrap()`s. At least when unwrapping, you don't have to think about why each individual [`Option`] is actually always `Some`.
//!
//! **You want to think about the substance of your test, not error handling boilerplate**
//!
//! # Wow, the `evil` crate is so cool! But Nightly Rust?
//!
//! This crate requires `nightly` rust. *But hold on!* That **does not** mean your project needs to have a `nightly` MSRV (Minimum Supported Rust Version).
//!
//! Your test suite's MSRV can be `nightly`, but your project's MSRV can be a stable Rust version. Tests aren't shipped to your users, so you're free to improve
//! your developer experience writing them as much as you'd like.
//!
//! When developing my Rust projects, I always have a `rust-toolchain.toml` that uses `nightly` Rust:
//!
//! ```toml
//! toolchain.channel = "nightly"
//! ```
//!
//! Then, in `Cargo.toml`, I set a stable MSRV:
//!
//! ```toml
//! [package]
//! rust-version = "1.90"
//! ```
//!
//! Now, all the Nightly Rust components will be used for tests. You get to use unstable features in tests all the time, while having the actual project build using Stable Rust. You get faster compile speeds. You get to use nightly [rustfmt options](https://rust-lang.github.io/rustfmt/) like [`wrap_comments`](https://rust-lang.github.io/rustfmt/?version=v1.9.0&search=#wrap_comments), [`format_code_in_doc_comments`](https://rust-lang.github.io/rustfmt/?version=v1.9.0&search=#format_code_in_doc_comments) and [`imports_granularity = "Item"`](https://rust-lang.github.io/rustfmt/?version=v1.9.0&search=#imports_granularity) for way less merge conflicts. Nightly compile speeds are faster, it's amazing for developing.
//!
//! But when it comes to shipping the code to users, the actual code will build on Stable Rust and not use any unstable features. I use [`cargo hack`](https://github.com/taiki-e/cargo-hack) in GitHub Actions CI to check that my project always builds with my MSRV:
//!
//! ```yml
//! # This GitHub action on every commit to the `main` branch,
//! # and on every Pull Request
//!
//! name: Check
//! on:
//!   pull_request:
//!   push:
//!     branches:
//!       - main
//!
//! jobs:
//!   cargo-check:
//!     runs-on: ubuntu-latest
//!     steps:
//!       - uses: actions/checkout@v6
//!       - uses: actions-rust-lang/setup-rust-toolchain@v1
//!       - uses: taiki-e/install-action@cargo-hack
//!
//!       - run: cargo hack check --each-feature --locked --rust-version --ignore-private --workspace --lib --bins --keep-going
//! ```
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(try_trait_v2)]
#![feature(never_type)]
#![allow(rustdoc::invalid_rust_codeblocks)]

use core::convert::Infallible;
use core::ops::ControlFlow;
use core::ops::FromResidual;
use core::ops::Try;

/// This is **always** [`Result::Ok`]
///
/// This is like [`core::result::Result`], but it's **impossible** to create a [`Result::Err`],
/// because when you use the `?` operator, it *panics*.
///
/// See the [crate-level](crate) documentation for more information.
pub enum Result<T = ()> {
    /// Contains the success value
    Ok(T),
    /// Contains the error value
    Err(!),
}

impl<T> Result<T> {
    /// Extracts the [`Result::Ok`] value. Always succeeds.
    pub fn into_ok(self) -> T {
        let Result::Ok(value) = self;
        value
    }
}

pub use Result::Ok;

/// Allows invoking the `?` operator on [`Option`], [`Result`], and [`evil::Result`](Result) from a try context of [`evil::Result`](Result)
impl<T> Try for Result<T> {
    type Output = T;
    type Residual = Result<Infallible>;

    /// `?` on a `T` to convert it into [`Result::Ok`]
    #[inline]
    fn from_output(output: Self::Output) -> Self {
        Self::Ok(output)
    }

    /// `?` on a [`Result`] to convert it into `T` or panic
    #[inline]
    #[track_caller]
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        let Result::Ok(value) = self;
        ControlFlow::Continue(value)
    }
}

/// You can invoke `?` on an [`Option`] from a try context of [`evil::Result`](Result)
impl<T> FromResidual<Option<Infallible>> for Result<T> {
    #[inline]
    #[track_caller]
    fn from_residual(_: Option<Infallible>) -> Self {
        panic!("`evil::Result`: invoked the `?` operator on an `Option::None` value")
    }
}

/// You can invoke `?` on a [`Result`](core::result::Result) from a try context of [`evil::Result`](Result)
impl<T, E: core::fmt::Debug> FromResidual<core::result::Result<Infallible, E>> for Result<T> {
    #[inline]
    #[track_caller]
    fn from_residual(residual: core::result::Result<Infallible, E>) -> Self {
        let core::result::Result::Err(err) = residual;
        panic!("invoked the `?` operator on a `Result::Err` value: {err:?}")
    }
}

/// You can invoke `?` on a [`evil::Result`](Result) from a try context of [`evil::Result`](Result)
impl<T> FromResidual<Result<Infallible>> for Result<T> {
    #[inline]
    #[track_caller]
    fn from_residual(residual: Result<Infallible>) -> Self {
        match residual {}
    }
}

/// You can invoke `?` on a [`evil::Result`](Result) from a try context of [`Result`](core::result::Result)
impl<T, E> FromResidual<Result<Infallible>> for core::result::Result<T, E> {
    fn from_residual(residual: Result<Infallible>) -> Self {
        match residual {}
    }
}

/// You can return [`evil::Result`](Result) from the `main` function and `#[test]`s
#[cfg(feature = "std")]
impl<T> std::process::Termination for Result<T> {
    fn report(self) -> std::process::ExitCode {
        std::process::ExitCode::SUCCESS
    }
}
