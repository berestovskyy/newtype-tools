Newtype Tools
=============

[![Discussions]][Discussions Link]
[![CI Status]][CI Link]
[![CD Status]][CD Link]
[![no_std Status]][no_std Link]
[![Coverage Status]][Coverage Link]
[![Docs.rs]][Docs.rs Link]
[![Crates.io]][Crates.io Link]

A lightweight library (~1K lines of code with minimum dependencies) designed to make
the [newtype idiom][newtype] more ergonomic to use.

Motivation
----------

Instead of trying to be everything or deriving dozens of unused trait implementations,
this crate provides unique, simple, yet powerful tools for the `newtypes`.

The crate focuses on three main areas to make `newtype` usage more enjoyable:

1. Conversions between types.
2. Operations on `newtypes`.
3. Iteration over `newtype` ranges.

Usage
-----

```bash
cargo add newtype-tools
```

Examples
--------

The simplest way to use the crate is to declare a tuple struct as a `newtype` kind:

```rust
# #[cfg(feature = "derive")]
# {
#[newtype_tools::newtype(Amount)]
struct Apples(u64);

// Now the `Apples`behave pretty much as their inner type `u64`:
let apple1 = Apples(2);
// `Apples` can be converted from the inner type:
let apple2 = Apples::from(3);
// `Apples` can be added, subtracted and compared:
assert_eq!(apple1 + apple2, Apples(5));
// `Apples` can be multiplied by the inner factor:
assert_eq!(apple1 * 2_u64, Apples(4));
// `Apples` can be divided, returning a inner ratio:
assert_eq!(apple2 / apple1 , 1);
// `Apples` can be easily extended:
impl core::fmt::Display for Apples {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.0, f)
    }
}
# }
```

The crate supports two kinds of `newtypes`: `Amount` and `Id`. See below for more details.

Rather than using the predefined sets of derives, the implementation allows
for the derivation of only the necessary traits. Conversion between types:

```rust
# #[cfg(feature = "derive")]
# {
use newtype_tools::Newtype;

#[derive(Newtype)]
#[newtype(
    into(Oranges, with = |apples| Oranges((apples.0 / 2) as u32))
)]
struct Apples(u64);
struct Oranges(u32);

let apples = Apples(42);
// `Oranges` can now be created from `Apples`:
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
    partial_eq(Oranges, with = |apples, oranges| apples.0 == oranges.0 as u64 * 2)
)]
struct Apples(u64);
struct Oranges(u32);

let apples = Apples(42);
let oranges = Oranges(21);

// `Apples` and `Oranges` can now be compared:
assert!(apples == oranges);
# }
```

Iterations over `newtype` ranges:

```rust
# #[cfg(feature = "derive")]
# {
use newtype_tools::{Newtype, Iter};

#[derive(Debug, Newtype)]
struct Apples(u64);

let range = Apples(0)..Apples(42);
// The range of `Apples` can now be iterated:
for apple in range.iter() {
    println!("{apple:?}");
}
# }
```

This will become even more ergonomic once the [Step][step] trait is stabilized.

Newtype Kinds
-------------

The crate supports predefined sets of newtype properties. The concept is similar
to the `phantom_newtype` crate but avoids its limitations, as the newtype
generated here is a distinct Rust type. This allows new traits
to be implemented easily for the type and makes the set of derived traits
simple to extend.

The supported `newtype` kinds are:

| Trait             | `#[newtype(Amount)]` | `#[newtype(Id)]` |
| ----------------- | :------------------: | :--------------: |
| `Clone`           |          ✔           |        ✔         |
| `Copy`            |          ✔           |        ✔         |
| `Debug`           |          ✔           |        ✔         |
| `Default`         |          ✔           |        ✔         |
| `Eq`¹             |          ✔           |        ✔         |
| `Hash`¹           |          ✔           |        ✔         |
| `Ord`¹            |          ✔           |        ✔         |
| `PartialEq`       |          ✔           |        ✔         |
| `PartialOrd`      |          ✔           |        ✔         |
| `From<Repr>`      |          ✔           |        ✔         |
| `Add<Self>`       |          ✔           |        ✘         |
| `AddAssign<Self>` |          ✔           |        ✘         |
| `Sub<Self>`       |          ✔           |        ✘         |
| `SubAssign<Self>` |          ✔           |        ✘         |
| `Mul<Repr>`       |          ✔           |        ✘         |
| `MulAssign<Repr>` |          ✔           |        ✘         |
| `Div<Self>`       |          ✔           |        ✘         |

1. `Eq`, `Ord` and `Hash` are only implemented for integer inner types.

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
[no_std Status]: https://github.com/berestovskyy/newtype-tools/actions/workflows/no_std.yaml/badge.svg?branch=main
[no_std Link]: https://github.com/berestovskyy/newtype-tools/actions/workflows/no_std.yaml?query=branch%3Amain
[Coverage Status]: https://codecov.io/github/berestovskyy/newtype-tools/branch/main/graph/badge.svg?token=EDGTFYZI3P
[Coverage Link]: https://codecov.io/github/berestovskyy/newtype-tools
[Docs.rs]: https://docs.rs/newtype-tools/badge.svg
[Docs.rs Link]: https://docs.rs/newtype-tools
[Crates.io]: https://img.shields.io/crates/v/newtype-tools.svg
[Crates.io Link]: https://crates.io/crates/newtype-tools
