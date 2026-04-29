Newtype Tools
=============

[![Discussions]][Discussions Link]
[![CI Status]][CI Link]
[![CD Status]][CD Link]
[![no_std Status]][no_std Link]
[![Coverage Status]][Coverage Link]
[![Docs.rs]][Docs.rs Link]
[![Crates.io]][Crates.io Link]
[![ChangeLog]][ChangeLog Link]

Ergonomic utilities for the Rust [newtype idiom][newtype].

Motivation
----------

This crate avoids the bloat of general-purpose "derive-all" libraries, offering instead
a minimalist set of powerful tools specifically for the [newtype idiom][newtype].

Development is focused on three core pillars:

1. Conversions between `newtypes` and between `newtypes` and their inner types.
2. Operations directly on `newtype` values.
3. Iteration over ranges of `newtypes`.

Usage
-----

```bash
cargo add newtype-tools
```

Examples
--------

The simplest way to use the crate is to define a tuple struct using the `newtype` attribute:

```rust
# #[cfg(feature = "derive")]
# {
// Derive `newtype` with `Amount` properties (see below for more details).
#[newtype_tools::newtype(Amount)]
// More traits can be easily derived:
#[derive(serde::Serialize)]
struct Apples(u64);

// The `newtype` can also be easily extended:
impl core::fmt::Display for Apples {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.0, f)
    }
}

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
# }
```

The crate supports two main "presets": `Amount` and `Id`. See below for more details.

Instead of using predefined sets, the implementation allows for the derivation
of only the specific traits required for a given use case.

**`newtype` conversions:**

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

**`newtype` operations:**

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

**`newtype` range iteration:**

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

Note: This will become even more ergonomic once the Rust [Step trait][step] is stabilized.

Newtype Kinds
-------------

The crate supports predefined sets of `newtype` properties. The concept is similar
to the `phantom_newtype` crate but avoids Rust orphan rule limitations by defining
the `newtype` locally. This allows additional traits to be implemented easily
and makes the set of derived traits simple to extend.

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

1. `derive_more` -- A large library (~10K SLoC) focused on supporting a wide range of traits
   for any data type. It can be used alongside `newtype-tools` to derive additional traits as needed:
   [crate](https://crates.io/crates/derive_more)
   | [repo](https://github.com/JelteF/derive_more).

   The unique value of this `newtype-tools` crate lies in its lightweight design,
   support for property sets like `#[newtype(Amount)]`, and built-in range iteration
   via `(Newtype(0)..Newtype(42)).iter()`.

2. `nutype` -- A comprehensive library (7.5K SLoC) primarily focused on `newtype` validation:
   [crate](https://crates.io/crates/nutype)
   | [repo](https://github.com/greyblake/nutype).

3. `phantom_newtype` -- Provides 19 trait implementations out of the box, but lacks a mechanism
   for custom trait implementations due to the Rust orphan rule:
   [crate](https://crates.io/crates/phantom_newtype)
   | [repo](https://github.com/roman-kashitsyn/phantom-newtype).

4. `newtype_derive` -- Legacy crate that relies on the `custom_derive!` declarative macro:
   [crate](https://crates.io/crates/newtype_derive)
   | [repo](https://github.com/DanielKeep/rust-custom-derive).

5. `newtype-derive-2018` -- Less outdated, but still based on the declarative macro:
   [crate](https://crates.io/crates/newtype-derive-2018)
   | [repo](https://github.com/A1-Triard/newtype-derive-2018).

6. `newtype` -- An older crate following a similar approach but no longer actively maintained:
   [crate](https://crates.io/crates/newtype)
   | [repo](https://gitlab.com/jrobsonchase/newtype).

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
[ChangeLog]: https://img.shields.io/badge/changelog-latest-blue.svg
[ChangeLog Link]: https://github.com/berestovskyy/newtype-tools/releases
