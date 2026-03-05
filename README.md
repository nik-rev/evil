# The `evil` crate 😈

<!-- cargo-reedme: start -->

<!-- cargo-reedme: info-start

    Do not edit this region by hand
    ===============================

    This region was generated from Rust documentation comments by `cargo-reedme` using this command:

        cargo +nightly reedme

    for more info: https://github.com/nik-rev/cargo-reedme

cargo-reedme: info-end -->

[![crates.io](https://img.shields.io/crates/v/evil?style=flat-square&logo=rust)](https://crates.io/crates/evil)
[![docs.rs](https://img.shields.io/docsrs/evil?style=flat-square&logo=docs.rs)](https://docs.rs/evil)
![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)
![msrv](https://img.shields.io/badge/msrv-nightly-blue?style=flat-square&logo=rust)
[![github](https://img.shields.io/github/stars/nik-rev/evil)](https://github.com/nik-rev/evil)

This crate lets you use the `?` operator as a shorthand for `.unwrap()`. Works on both [`Result`](https://doc.rust-lang.org/stable/core/result/enum.Result.html) and [`Option`](https://doc.rust-lang.org/stable/core/option/enum.Option.html)!

```toml
evil = "0.2"
```

## Example

The `evil` crate *significantly* reduces boilerplate in tests. Error handling *in tests* dilutes the substance of your test.

By removing all that boilerplate, you are now free to spend your energy and focus on what you are *actually* testing.

### Before

```rust
#[test]
fn user_theme_preference() {
    let response = make_api_call("/user/profile/settings").unwrap();
    let json: Value = serde_json::from_str(&response).unwrap();

    let theme = json
        .get("data")
        .unwrap()
        .get("attributes")
        .unwrap()
        .get("preferences")
        .unwrap()
        .get("theme")
        .unwrap()
        .as_str()
        .unwrap();

    assert_eq!(theme, "dark");
}
```

### After

Use [`evil::Result<()>`](https://docs.rs/evil/latest/evil/enum.Result.html) as the return type of your test functions:

```rust
#[test]
fn user_theme_preference() -> evil::Result<()> {
    let response = make_api_call("/user/profile/settings")?;
    let json: Value = serde_json::from_str(&response)?;

    let theme = json
        .get("data")?
        .get("attributes")?
        .get("preferences")?
        .get("theme")?
        .as_str()?;

    assert_eq!(theme, "dark");
    evil::Ok(())
}
```

Each one of those `?` is equivalent to a `.unwrap()`.

## Use the `evil` crate in scripts

When writing small Rust scripts that will only be used by developers, `.unwrap()`ping everything instead of proper error handling is common.

**But there is one huge disadvantage with that approach.**

Scripts turn into programs much more often than we’d like. Then, refactoring all of that `.unwrap()` boilerplate into good error handling is a significant undertaking.

If you use [`evil::Result<()>`](https://docs.rs/evil/latest/evil/enum.Result.html) from the get-go, later refactoring your script to use something like [`anyhow::Result<()>`](https://docs.rs/anyhow/latest/anyhow/type.Result.html) is much simpler - you’re
already using the `?` operator *everywhere* anyway. It’s a piece of cake.

## Why should I use [`evil::Result<()>`](https://docs.rs/evil/latest/evil/enum.Result.html) instead of [`eyre::Result<()>`](https://docs.rs/eyre/latest/eyre/type.Result.html)?

The benefits of unwrapping everything is that you get the exact file, line and column information on where the unwrap failed. That’s **amazing**. It helps debugging tremendously.

When returning <code>Result\<(), Box\<dyn [core::error::Error](https://doc.rust-lang.org/stable/core/error/trait.Error.html)\>\></code> from your function, you don’t get that. That information is simply *discarded*. Good luck figuring out where the error came from if you just use `?`. When returning [`anyhow::Result<()>`](https://docs.rs/anyhow/latest/anyhow/type.Result.html), it’s the same problem.

**But [`eyre::Result<()>`](https://docs.rs/eyre/latest/eyre/type.Result.html) is built different. It is *special*.**

<br>

`eyre::Result<()>` actually tells you the file, line and column information of where you use the `?` operator. But it has one *huge* downside compared to `evil::Result<()>`: **It only works on `Result`s, not `Option`s.**

<br>

Let’s come back to our example and rewrite it with `eyre`:

```rust
use eyre::OptionExt as _;

#[test]
fn user_theme_preference() -> eyre::Result<()> {
    let response = make_api_call("/user/profile/settings")?;
    let json: Value = serde_json::from_str(&response)?;

    let theme = json
        .get("data")
        .ok_or_eyre("I have to give a reason why this is not `None`")?
        .get("attributes")
        .ok_or_eyre("and for this one as well...")?
        .get("preferences")
        .ok_or_eyre(".....I'm getting tired of this.....")?
        .get("theme")
        .ok_or_eyre("...............")
        .as_str()
        .ok_or_eyre(":/");

    assert_eq!(theme, "dark");
    Ok(())
}
```

This is **even more verbose** than just using `.unwrap()`s. At least when unwrapping, you don’t have to think about why each individual [`Option`](https://doc.rust-lang.org/stable/core/option/enum.Option.html) is actually always `Some`.

**You want to think about the substance of your test, not error handling boilerplate**

## Wow, the `evil` crate is so cool! But Nightly Rust?

This crate requires `nightly` rust, because customizing behavior of the `?` operator requires the [`Try`](https://doc.rust-lang.org/stable/core/ops/try_trait/trait.Try.html) trait.

*But hold on!* That **does not** mean your project needs to have a `nightly` MSRV (Minimum Supported Rust Version).

Your test suite’s MSRV can be `nightly`, but your project’s MSRV can be a stable Rust version. Tests aren’t shipped to your users, so you’re free to improve
your developer experience writing them as much as you’d like.

When developing my Rust projects, I always have a `rust-toolchain.toml` that uses `nightly` Rust:

```toml
toolchain.channel = "nightly"
```

Then, in `Cargo.toml`, I set a stable MSRV:

```toml
[package]
rust-version = "1.90"
```

Now, all the Nightly Rust components will be used for tests. You get to use unstable features in tests all the time, while having the actual project build using Stable Rust. You get faster compile speeds. You get to use nightly [rustfmt options](https://rust-lang.github.io/rustfmt/) like [`wrap_comments`](https://rust-lang.github.io/rustfmt/?version=v1.9.0&search=#wrap_comments), [`format_code_in_doc_comments`](https://rust-lang.github.io/rustfmt/?version=v1.9.0&search=#format_code_in_doc_comments) and [`imports_granularity = "Item"`](https://rust-lang.github.io/rustfmt/?version=v1.9.0&search=#imports_granularity) for way less merge conflicts. Nightly compile speeds are faster, it’s amazing for developing.

But when it comes to shipping the code to users, the actual code will build on Stable Rust and not use any unstable features. I use [`cargo hack`](https://github.com/taiki-e/cargo-hack) in GitHub Actions CI to check that my project always builds with my MSRV:

```yml
# This GitHub action runs on every commit to the `main` branch,
# and also on every Pull Request

name: Check
on:
  pull_request:
  push:
    branches:
      - main

jobs:
  cargo-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6
      - uses: actions-rust-lang/setup-rust-toolchain@v1
      - uses: taiki-e/install-action@cargo-hack

      - run: cargo hack check --each-feature --locked --rust-version --ignore-private --workspace --lib --bins --keep-going
```

## How does it work?

The `?` operator is syntax sugar for the [`Try`](https://doc.rust-lang.org/stable/core/ops/try_trait/trait.Try.html) trait, plus its friends [`ControlFlow`](https://doc.rust-lang.org/stable/core/ops/control_flow/enum.ControlFlow.html) and [`FromResidual`](https://doc.rust-lang.org/stable/core/ops/try_trait/trait.FromResidual.html).

Consider:

```rust
let html = fetch()?;
```

The above desugars to the following:

```rust
let html = match Try::branch(fetch()) {
    ControlFlow::Continue(v) => v,
    ControlFlow::Break(r) => return FromResidual::from_residual(r),
};
```

The return type of `fetch()` must implement the `Try` trait. In our example, `fetch()` returns `Result<String, FetchError>`.

This is the implementation of `Try` for `Result`, which comes from the standard library:

```rust
impl<T, E> Try for Result<T, E> {
    type Residual = Result<!, E>;

    fn branch(self) -> ControlFlow<Self::Residual, T> {
        match self {
            Ok(c) => ControlFlow::Continue(c),
            Err(e) => ControlFlow::Break(Result::<!, E>::Err(e)),
        }
    }
}
```

> **What is that `type Residual`?**
>
> The “residual” is an “always-fail” version of a type implementing `Try`. For example:
>
> - For any `Option<T>`, the residual is `Option<!>`, which is always just `None` - because `Option::None` is considered the failure case of an `Option`. An `Option<!>` has no `Option::Some`, which means it is always `Option::None`
> - For any `Result<T, E>`, the residual is `Result<!, E>` - because `Result::Err` is the failure case of a `Result`. A `Result<!, E>` has no `Result::Ok`, which means it is always `Result::Err`

The `fetch()` call **failed** and it returned `Result::<String, FetchError>::Err(FetchError)`. Our `match` simplifies to:

```rust
let html = match ControlFlow::Break(match Result::<String, FetchError>::Err(FetchError) {
    Ok(c) => ControlFlow::Continue(c),
    Err(e) => ControlFlow::Break(Result::<!, E>::Err(e)),
}) {
    ControlFlow::Continue(v) => v,
    ControlFlow::Break(r) => return FromResidual::from_residual(r),
};
```

Which then simplifies to:

```rust
let html = match ControlFlow::Break(Result::<!, FetchError>::Err(FetchError)) {
    ControlFlow::Continue(v) => v,
    ControlFlow::Break(r) => return FromResidual::from_residual(r),
};
```

Which then simplifies to:

```rust
let html = return FromResidual::from_residual(Result::<!, FetchError>::Err(FetchError));
```

We hit an error, and we do an **early return**. This is the short-circuiting behavior of the `?` operator.

Now consider that the function we are inside returns a `evil::Result<()>`:

```rust
fn process_webpage() -> evil::Result<()> {
    let html = return FromResidual::from_residual(Result::<!, FetchError>::Err(FetchError));
}
```

The `FromResidual` trait is generic. In the above example, the generic type parameter has been inferred to be whatever `from_residual` function needs to return.

Let’s explicitly insert the inferred type, `evil::Result<()>`:

```rust
fn process_webpage() -> evil::Result<()> {
    let html = return FromResidual::<evil::Result<()>>::from_residual(Result::<!, FetchError>::Err(FetchError));
}
```

We cannot just return `Result<!, FetchError>` from the function, because it is a completely different type to `evil::Result<()>`. We must figure out how to *convert* from the former to the latter.

That’s where the `FromResidual` trait comes into play. It does just that.

The implementation of `FromResidual` that gets used above is the following:

```rust
impl<T, E: Debug> FromResidual<Result<!, E>> for evil::Result<T> {
    fn from_residual(residual: Result<!, E>) -> Self {
        // ...
    }
}
```

That `from_residual` is the magic sauce that tells us how we go from `Result<!, FetchError>` to `evil::Result<()>`.

Whatever `from_residual` returns, *that’s* what we will return from `process_webpage` function.

When you use `?` on a `Result` type in a function that returns `Result`, the standard library implementation is used:

```rust
impl<T, E, F: From<E>> FromResidual<Result<!, E>> for Result<T, F> {
    fn from_residual(residual: Result<!, E>) -> Self {
        match residual {
            Err(e) => Err(From::from(e)),
        }
    }
}
```

However, in our example, we’re not converting from `Result` to `Result`. We’re converting from `Result` to `evil::Result`, where a slightly different implementation is used:

```rust
impl<T, E: Debug> FromResidual<Result<!, E>> for evil::Result<T> {
    #[track_caller]
    fn from_residual(residual: Result<!, E>) -> Self {
        match residual {
            Err(e) => Err(panic!("invoked `?` on an `Err` value: {e:?}"))
        }
    }
}
```

The only bit that’s different here is that instead of returning `Result::Err`, we **panic**.

Conceptually, `panic!()` “returns” the [`!`](https://doc.rust-lang.org/stable/std/primitive.never.html) type, hence `evil::Result` is defined as follows:

```rust
pub enum Result<T> {
    Ok(T),
    Err(!),
}
```

<!-- cargo-reedme: end -->
