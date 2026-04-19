Newtype Tools
=============

[![Discussions]][Discussions Link]
[![CI Status]][CI Link]
[![CD Status]][CD Link]
[![Coverage Status]][Coverage Link]
[![Docs.rs]][Docs.rs Link]
[![Crates.io]][Crates.io Link]

A lightweight library (~1K lines of code with minimum dependencies) designed to make
the [newtype idiom][newtype] more ergonomic to use.

Motivation
----------

Instead of trying to be everything or deriving dozens of unused trait implementations,
this crate provides unique, simple, yet powerful tools for your `newtypes`.

The crate focuses on three main areas to make `newtype` usage more enjoyable:

1. Conversions between types.
2. Operations on `newtypes`.
3. Iteration over `newtype` ranges.

Usage
-----

Adding the crate to your project:

```bash
cargo add newtype-tools
```

Examples
--------

Conversion between types:

```rust
# #[cfg(feature = "derive")]
# {
use newtype_tools::Newtype;

#[derive(Newtype)]
#[newtype(
    from(Oranges, with = "|oranges| Apples(oranges.0 as u64 * 2)"),
    into(Oranges, with = "|apples| Oranges((apples.0 / 2) as u32)")
)]
struct Apples(u64);
struct Oranges(u32);

let apples = Apples(42);
assert_eq!(apples.0, 42);

let oranges = Oranges::from(apples);
assert_eq!(oranges.0, 21);
# }
```

Operations on `newtypes`:

```rust
# #[cfg(feature = "derive")]
# {
use newtype_tools::Newtype;

#[derive(Debug, Newtype)]
#[newtype(
    partial_eq(Oranges, with = "|apples, oranges| apples.0 == oranges.0 as u64 * 2")
)]
struct Apples(u64);
#[derive(Debug)]
struct Oranges(u32);

let apples = Apples(42);
let oranges = Oranges(21);
assert_eq!(apples, oranges);
# }
```

Iterations over `newtype` ranges:

```rust
# #[cfg(feature = "derive")]
# {
use newtype_tools::{Newtype, Iterator};

#[derive(Debug, Newtype)]
struct Apples(u64);

let range = Apples(0)..Apples(42);
for apple in Iterator::from(&range) {
    println!("{apple:?}");
}
# }
```

This will become even more ergonomic once the [Step][step] trait is stabilized.

Alternatives
------------

1. `nutype` -- An impressive 12.5k lines of code, with 7.5k lines in proc-macros alone.
   After trying to extend it, I realized it would be faster to simply write a new crate.
2. `phantom_newtype` -- Provides 19 trait implementations out of the box,
   but lacks a mechanism for providing custom trait implementations.
3. `newtype_derive` -- Outdated and relies on the legacy `custom_derive!` declarative macro.
4. `newtype-derive-2018` -- Less outdated, but it's 2026.

References
----------

1. Rust [newtype idiom][newtype].
2. Rust [Step trait][step].

[newtype]: https://doc.rust-lang.org/rust-by-example/generics/new_types.html
[step]: https://doc.rust-lang.org/std/iter/trait.Step.html

[Discussions]: https://img.shields.io/github/discussions/berestovskyy/newtype-tools?color=blueviolet
[Discussions Link]: https://github.com/berestovskyy/newtype-tools/discussions
[CI Status]: https://github.com/berestovskyy/newtype-tools/actions/workflows/ci.yaml/badge.svg?branch=main
[CI Link]: https://github.com/berestovskyy/newtype-tools/actions/workflows/ci.yaml?query=branch%3Amain
[CD Status]: https://github.com/berestovskyy/newtype-tools/actions/workflows/cd.yaml/badge.svg?branch=main
[CD Link]: https://github.com/berestovskyy/newtype-tools/actions/workflows/cd.yaml?query=branch%3Amain
[Coverage Status]: https://codecov.io/github/berestovskyy/newtype-tools/branch/main/graph/badge.svg?token=EDGTFYZI3P
[Coverage Link]: https://codecov.io/github/berestovskyy/newtype-tools
[Docs.rs]: https://docs.rs/newtype-tools/badge.svg
[Docs.rs Link]: https://docs.rs/newtype-tools
[Crates.io]: https://img.shields.io/crates/v/newtype-tools.svg
[Crates.io Link]: https://crates.io/crates/newtype-tools
