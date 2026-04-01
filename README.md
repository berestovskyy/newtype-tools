Newtype Tools
=============

A lightweight library (~600 lines of code with minimum dependencies) designed to make
the [newtype idiom][newtype] more ergonomic to use.

Motivation
----------

Instead of trying to be everything or deriving dozens of unused trait implementations,
this crate provides unique, simple, yet powerful tools for your `newtypes`.

The crate focuses on three main areas to make `newtype` usage more enjoyable:

1. Conversions between types.
2. Operations on `newtypes`.
3. Iteration over `newtypes` ranges.

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
use newtype_tools::Newtype;

#[derive(Debug, Newtype)]
#[newtype(from(usize, with = |u| Apples(u as u64)))]
#[newtype(range_iter(usize))]
struct Apples(u64);

for apple in Apples::range_iter(Apples(0)..Apples(42)) {
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
