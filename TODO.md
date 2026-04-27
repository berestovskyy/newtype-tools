TODO
====

Some ideas for future development:

Convenient Aliases
------------------

Add optional `alias = "method_name"` argument to create an alias for a conversion implementation:

```rust
#[newtype(into(Oranges, alias = "into_oranges" with = "..."))]
struct Apples(u64);

let apple = Apples(1);
let oranges = apple.into_oranges();
```

Common Base Type Conversions
----------------------------

Similar to the convenient aliases above, but instead of a method name, a base type is specified:

```rust
#[newtype(
    base = "NumBytes",
    from(NumBytes, with = "...")
)]
struct NumOsPages(u64);

#[newtype(
    base = "NumBytes",
    from(NumBytes, with = "...")
)]
struct NumWasmPages(u64);

let num_os_pages = NumOsPages(1);
let num_wasm_pages = num_os_pages.into();
```

Support `newtype` Arrays and Vectors
------------------------------------

Something like:

```rust
#[newtype(Array)]
struct Apples([u64; 2]);
#[newtype(Vec)]
struct Oranges(Vec<u64>);
```

It's not quite clear what it should derive by default:

1. `Index` -- make sense as a custom derive, i.e. `#newtype(index(...))`
2. `FromIterator`?
3. `Extend`?

There is no clear use case at the moment.

Extend Pattern
--------------

Consider a use case of extending functionality. See the `extend` crate:
https://crates.io/crates/extend
