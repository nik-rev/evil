#![doc = concat!("[![crates.io](https://img.shields.io/crates/v/", env!("CARGO_PKG_NAME"), "?style=flat-square&logo=rust)](https://crates.io/crates/", env!("CARGO_PKG_NAME"), ")")]
#![doc = concat!("[![docs.rs](https://img.shields.io/docsrs/", env!("CARGO_PKG_NAME"), "?style=flat-square&logo=docs.rs)](https://docs.rs/", env!("CARGO_PKG_NAME"), ")")]
#![doc = "![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)"]
//! ![msrv](https://img.shields.io/badge/msrv-nightly-blue?style=flat-square&logo=rust)
//! [![github](https://img.shields.io/github/stars/nik-rev/evil)](https://github.com/nik-rev/evil)
//!
//! This crate lets you use the `?` operator as a shorthand for `.unwrap()`. Works on both [`Result`](core::result::Result) and [`Option`](core::option::Option)!
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
//! Use [`evil::Result<()>`](Result) as the return type of your test functions:
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
//! This crate requires `nightly` rust, because customizing behavior of the `?` operator requires the [`Try`] trait.
//!
//! *But hold on!* That **does not** mean your project needs to have a `nightly` MSRV (Minimum Supported Rust Version).
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
//! # This GitHub action runs on every commit to the `main` branch,
//! # and also on every Pull Request
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
//!
//! # How does it work?
//!
//! The `?` operator is syntax sugar for the [`Try`] trait, plus its friends [`ControlFlow`] and [`FromResidual`].
//!
//! Consider:
//!
//! ```
//! # /*
//! let html = fetch()?;
//! # */
//! ```
//!
//! The above desugars to the following:
//!
//! ```
//! # /*
//! let html = match Try::branch(fetch()) {
//!     ControlFlow::Continue(v) => v,
//!     ControlFlow::Break(r) => return FromResidual::from_residual(r),
//! };
//! # */
//! ```
//!
//! The return type of `fetch()` must implement the `Try` trait. In our example, `fetch()` returns `Result<String, FetchError>`.
//!
//! This is the implementation of `Try` for `Result`, which comes from the standard library:
//!
//! ```
//! # /*
//! impl<T, E> Try for Result<T, E> {
//!     type Residual = Result<!, E>;
//!
//!     fn branch(self) -> ControlFlow<Self::Residual, T> {
//!         match self {
//!             Ok(c) => ControlFlow::Continue(c),
//!             Err(e) => ControlFlow::Break(Result::<!, E>::Err(e)),
//!         }
//!     }
//! }
//! # */
//! ```
//!
//! > **What is that `type Residual`?**
//! >
//! > The "residual" is an "always-fail" version of a type implementing `Try`. For example:
//! >
//! > - For any `Option<T>`, the residual is `Option<!>`, which is always just `None` - because `Option::None` is considered the failure case of an `Option`. An `Option<!>` has no `Option::Some`, which means it is always `Option::None`
//! > - For any `Result<T, E>`, the residual is `Result<!, E>` - because `Result::Err` is the failure case of a `Result`. A `Result<!, E>` has no `Result::Ok`, which means it is always `Result::Err`
//!
//! The `fetch()` call **failed** and it returned `Result::<String, FetchError>::Err(FetchError)`. Our `match` simplifies to:
//!
//! ```
//! # /*
//! let html = match ControlFlow::Break(match Result::<String, FetchError>::Err(FetchError) {
//!     Ok(c) => ControlFlow::Continue(c),
//!     Err(e) => ControlFlow::Break(Result::<!, E>::Err(e)),
//! }) {
//!     ControlFlow::Continue(v) => v,
//!     ControlFlow::Break(r) => return FromResidual::from_residual(r),
//! };
//! # */
//! ```
//!
//! Which then simplifies to:
//!
//! ```
//! # /*
//! let html = match ControlFlow::Break(Result::<!, FetchError>::Err(FetchError)) {
//!     ControlFlow::Continue(v) => v,
//!     ControlFlow::Break(r) => return FromResidual::from_residual(r),
//! };
//! # */
//! ```
//!
//! Which then simplifies to:
//!
//! ```
//! # /*
//! let html = return FromResidual::from_residual(Result::<!, FetchError>::Err(FetchError));
//! # */
//! ```
//!
//! We hit an error, and we do an **early return**. This is the short-circuiting behavior of the `?` operator.
//!
//! Now consider that the function we are inside returns a `evil::Result<()>`:
//!
//! ```
//! # /*
//! fn process_webpage() -> evil::Result<()> {
//!     let html = return FromResidual::from_residual(Result::<!, FetchError>::Err(FetchError));
//! }
//! # */
//! ```
//!
//! The `FromResidual` trait is generic. In the above example, the generic type parameter has been inferred to be whatever `from_residual` function needs to return.
//!
//! Let's explicitly insert the inferred type, `evil::Result<()>`:
//!
//! ```
//! # /*
//! fn process_webpage() -> evil::Result<()> {
//!     let html = return FromResidual::<evil::Result<()>>::from_residual(Result::<!, FetchError>::Err(FetchError));
//! }
//! # */
//! ```
//!
//! We cannot just return `Result<!, FetchError>` from the function, because it is a completely different type to `evil::Result<()>`. We must figure out how to *convert* from the former to the latter.
//!
//! That's where the `FromResidual` trait comes into play. It does just that.
//!
//! The implementation of `FromResidual` that gets used above is the following:
//!
//! ```
//! # /*
//! impl<T, E: Debug> FromResidual<Result<!, E>> for evil::Result<T> {
//!     fn from_residual(residual: Result<!, E>) -> Self {
//!         // ...
//!     }
//! }
//! # */
//! ```
//!
//! That `from_residual` is the magic sauce that tells us how we go from `Result<!, FetchError>` to `evil::Result<()>`.
//!
//! Whatever `from_residual` returns, *that's* what we will return from `process_webpage` function.
//!
//! When you use `?` on a `Result` type in a function that returns `Result`, the standard library implementation is used:
//!
//! ```
//! # /*
//! impl<T, E, F: From<E>> FromResidual<Result<!, E>> for Result<T, F> {
//!     fn from_residual(residual: Result<!, E>) -> Self {
//!         match residual {
//!             Err(e) => Err(From::from(e)),
//!         }
//!     }
//! }
//! # */
//! ```
//!
//! However, in our example, we're not converting from `Result` to `Result`. We're converting from `Result` to `evil::Result`, where a slightly different implementation is used:
//!
//! ```
//! # /*
//! impl<T, E: Debug> FromResidual<Result<!, E>> for evil::Result<T> {
//!     #[track_caller]
//!     fn from_residual(residual: Result<!, E>) -> Self {
//!         match residual {
//!             Err(e) => Err(panic!("invoked `?` on an `Err` value: {e:?}"))
//!         }
//!     }
//! }
//! # */
//! ```
//!
//! The only bit that's different here is that instead of returning `Result::Err`, we **panic**.
//!
//! Conceptually, `panic!()` "returns" the [`!`] type, hence `evil::Result` is defined as follows:
//!
//! ```
//! # /*
//! pub enum Result<T> {
//!     Ok(T),
//!     Err(!),
//! }
//! # */
//! ```
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(try_trait_v2)]
#![feature(never_type)]
#![allow(rustdoc::invalid_rust_codeblocks)]

use core::convert::Infallible;
use core::fmt;
use core::fmt::Debug;
use core::hash::Hash;
use core::hash::Hasher;
use core::iter::Product;
use core::iter::Sum;
use core::ops::ControlFlow;
use core::ops::FromResidual;
use core::ops::Try;
use core::task::Poll;

/// This is like [`core::result::Result`], but it's **impossible** to create an [`Err`](Result::Err) value,
/// because when you use the `?` operator, it *panics*.
///
/// This is **always** an [`Ok`] value.
///
/// See the [crate-level](crate) documentation for more information.
///
/// # Definition
///
/// This `Result` is defined as a 2-variant `enum`, where the `Err` variant is *uninhabited* - it can never exist.
///
/// Why not represent `Result` as a newtype struct, then?
///
/// ```
/// struct Result<T>(T);
/// ```
///
/// Because that doesn't convey the semantics of "this is exactly like [`Result`], but it can never be [`Result::Err`]"
///
/// The `!` type is fantastic for use in documentation, and making the semantics of types clearer. That is exactly how we're using it here.
///
/// Consider a function that starts a web server, and only returns when an error is encountered:
///
/// ```
/// # /*
/// fn start_server(&self) -> HttpError {
/// # */
/// ```
///
/// It's not exactly obvious from the signature that this function really only returns an error, which means
/// it can actually never return. Consider a refined signature, using the `!` type:
///
/// ```
/// # /*
/// fn start_server(&self) -> Result<!, HttpError> {
/// # */
/// ```
///
/// This time, the signature itself explains a lot more.
///
/// Despite `HttpError` and `Result<!, HttpError>` return types both having the same representation,
/// the latter semantically makes more sense.
pub enum Result<T = ()> {
    /// Contains the success value
    Ok(T),
    /// Contains the error value
    Err(!),
}

pub use Result::Ok;

impl<T> Result<T> {
    /// Returns a reference to the [`Result::Ok`] value. Always succeeds.
    #[inline]
    pub fn as_ok(&self) -> &T {
        match self {
            Self::Ok(value) => value,
            Self::Err(error) => match *error {},
        }
    }

    /// Returns a mutable reference to the [`Result::Ok`] value. Always succeeds.
    #[inline]
    pub fn as_ok_mut(&mut self) -> &mut T {
        match self {
            Self::Ok(value) => value,
            Self::Err(error) => match *error {},
        }
    }

    /// Returns the [`Result::Ok`] value. Always succeeds.
    #[inline]
    pub fn into_ok(self) -> T {
        let Result::Ok(value) = self;
        value
    }
}

impl<T> Try for Result<T> {
    type Output = T;
    type Residual = Result<Infallible>;

    #[inline]
    fn from_output(output: Self::Output) -> Self {
        Self::Ok(output)
    }

    #[inline]
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        let Result::Ok(value) = self;
        ControlFlow::Continue(value)
    }
}

impl<T> FromResidual<Option<Infallible>> for Result<T> {
    #[inline]
    #[track_caller]
    fn from_residual(_: Option<Infallible>) -> Self {
        panic!("invoked `?` on a `None` value")
    }
}

impl<T, E: Debug> FromResidual<core::result::Result<Infallible, E>> for Result<T> {
    #[inline]
    #[track_caller]
    fn from_residual(residual: core::result::Result<Infallible, E>) -> Self {
        let core::result::Result::Err(err) = residual;
        panic!("invoked `?` on an `Err` value: {err:?}")
    }
}

impl<T> FromResidual<Result<Infallible>> for Result<T> {
    #[inline]
    fn from_residual(residual: Result<Infallible>) -> Self {
        match residual {}
    }
}

impl<T, E> FromResidual<Result<Infallible>> for core::result::Result<T, E> {
    #[inline]
    fn from_residual(residual: Result<Infallible>) -> Self {
        match residual {}
    }
}

impl<T> FromResidual<Result<Infallible>> for Poll<Option<Result<T>>> {
    #[inline]
    fn from_residual(x: Result<Infallible>) -> Self {
        match x {}
    }
}

impl<T, E> FromResidual<Result<Infallible>> for Poll<Option<core::result::Result<T, E>>> {
    #[inline]
    fn from_residual(x: Result<Infallible>) -> Self {
        match x {}
    }
}

impl<T> FromResidual<Result<Infallible>> for Poll<Result<T>> {
    #[inline]
    fn from_residual(x: Result<Infallible>) -> Self {
        match x {}
    }
}

impl<T, E> FromResidual<Result<Infallible>> for Poll<core::result::Result<T, E>> {
    #[inline]
    fn from_residual(x: Result<Infallible>) -> Self {
        match x {}
    }
}

impl<T: Copy> Copy for Result<T> {}

impl<T: Hash> Hash for Result<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ok().hash(state);
    }
}

impl<T: Debug> Debug for Result<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result {
        <T as Debug>::fmt(self.as_ok(), f)
    }
}

#[cfg(feature = "std")]
use std::process::ExitCode;
#[cfg(feature = "std")]
use std::process::Termination;

#[cfg(feature = "std")]
impl<T: Termination> Termination for Result<T> {
    #[inline]
    fn report(self) -> ExitCode {
        T::report(self.into_ok())
    }
}

impl<T> Clone for Result<T>
where
    T: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self::Ok(self.as_ok().clone())
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        self.as_ok_mut().clone_from(source.as_ok());
    }
}

impl<A, V: FromIterator<A>> FromIterator<Result<A>> for Result<V> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = Result<A>>>(iter: I) -> Result<V> {
        Self::Ok(iter.into_iter().map(|value| value.into_ok()).collect())
    }
}

impl<A, E: Debug, V: FromIterator<A>> FromIterator<core::result::Result<A, E>> for Result<V> {
    #[track_caller]
    #[inline]
    fn from_iter<I: IntoIterator<Item = core::result::Result<A, E>>>(iter: I) -> Result<V> {
        let values = iter.into_iter().map(|item| {
            match item {
                core::result::Result::Ok(value) => value,
                core::result::Result::Err(err) => {
                    panic!("failed to collect, encountered an `Err` value: {err:?}")
                }
            }
        });

        Result::Ok(V::from_iter(values))
    }
}

impl<A, V: FromIterator<A>> FromIterator<Option<A>> for Result<V> {
    #[track_caller]
    #[inline]
    fn from_iter<I: IntoIterator<Item = Option<A>>>(iter: I) -> Result<V> {
        let values = iter.into_iter().map(|item| {
            match item {
                Some(value) => value,
                None => {
                    panic!("failed to collect, encountered a `None` value")
                }
            }
        });

        Result::Ok(V::from_iter(values))
    }
}

impl<A, V: Product<A>> Product<Result<A>> for Result<V> {
    #[inline]
    fn product<I: Iterator<Item = Result<A>>>(iter: I) -> Self {
        Self::Ok(iter.into_iter().map(|value| value.into_ok()).product())
    }
}

impl<A, E: Debug, V: Product<A>> Product<core::result::Result<A, E>> for Result<V> {
    #[track_caller]
    #[inline]
    fn product<I: IntoIterator<Item = core::result::Result<A, E>>>(iter: I) -> Self {
        let values = iter.into_iter().map(|item| {
            match item {
                core::result::Result::Ok(value) => value,
                core::result::Result::Err(err) => {
                    panic!("failed to compute product, encountered an `Err` value: {err:?}")
                }
            }
        });

        Result::Ok(V::product(values))
    }
}

impl<A, V: Product<A>> Product<Option<A>> for Result<V> {
    #[track_caller]
    #[inline]
    fn product<I: Iterator<Item = Option<A>>>(iter: I) -> Self {
        let values = iter.into_iter().map(|item| {
            match item {
                Some(value) => value,
                None => {
                    panic!("failed to compute product, encountered a `None` value")
                }
            }
        });

        Result::Ok(V::product(values))
    }
}

impl<A, V: Sum<A>> Sum<Result<A>> for Result<V> {
    #[inline]
    fn sum<I: Iterator<Item = Result<A>>>(iter: I) -> Self {
        Self::Ok(iter.into_iter().map(|value| value.into_ok()).sum())
    }
}

impl<A, E: Debug, V: Sum<A>> Sum<core::result::Result<A, E>> for Result<V> {
    #[track_caller]
    #[inline]
    fn sum<I: IntoIterator<Item = core::result::Result<A, E>>>(iter: I) -> Self {
        let values = iter.into_iter().map(|item| {
            match item {
                core::result::Result::Ok(value) => value,
                core::result::Result::Err(err) => {
                    panic!("failed to compute sum, encountered an `Err` value: {err:?}")
                }
            }
        });

        Result::Ok(V::sum(values))
    }
}

impl<A, V: Sum<A>> Sum<Option<A>> for Result<V> {
    #[track_caller]
    #[inline]
    fn sum<I: Iterator<Item = Option<A>>>(iter: I) -> Self {
        let values = iter.into_iter().map(|item| {
            match item {
                Some(value) => value,
                None => {
                    panic!("failed to compute sum, encountered a `None` value")
                }
            }
        });

        Result::Ok(V::sum(values.into_iter()))
    }
}

impl<'a, T> IntoIterator for &'a Result<T> {
    type Item = &'a T;

    type IntoIter = core::array::IntoIter<&'a T, 1>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        [self.as_ok()].into_iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Result<T> {
    type Item = &'a mut T;

    type IntoIter = core::array::IntoIter<&'a mut T, 1>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        [self.as_ok_mut()].into_iter()
    }
}

impl<T> IntoIterator for Result<T> {
    type Item = T;

    type IntoIter = core::array::IntoIter<T, 1>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        [self.into_ok()].into_iter()
    }
}

impl<T: PartialEq> PartialEq for Result<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_ok().eq(other.as_ok())
    }
}

impl<T: Eq> Eq for Result<T> {}

impl<T: PartialOrd> PartialOrd for Result<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.as_ok().partial_cmp(other.as_ok())
    }
}

impl<T: Ord> Ord for Result<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_ok().cmp(other.as_ok())
    }
}
