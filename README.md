# `parameterized_test::create!()` macro

This small crate provides a `parameterized_test::create!()` macro to simplify creating repeated
tests with different arguments.

Inspired by [Chris Morgan's StackOverflow post](https://stackoverflow.com/a/34666891/113632) and
originally documented in [this answer](https://stackoverflow.com/a/56663823/113632), this macro
works by dynamically generating a new macro which, in turn, generates separate tests for each test
case passed to the generated macro.

**Note:** the exact API is still in development and may change in subsequent (pre-1.0) releases.

## Syntax

`parameterized_test::create()` expects four arguments:

* A name for the test group, which will be used as the submodule name and the name of the generated
  parameters macro.
* One or more variable names, e.g. `foo` or `(bar, baz)` (note multiple variables must be
  parenthesized).
* The test body, multiple statements can be enclosed in `{ ... }`. 

## Example

This example creates two test cases, `tests::even::bad_case` and `tests::even::good_case`.

```rust
use parameterized_test::create;

#[cfg(test)]
mod tests {
    use super::*;

    parameterized_test::create!{ even, n, { assert_eq!(n % 2, 0); } }
    even! {
        bad_case:  1, // this test case will fail
        good_case: 2,
    }
}
```

This example accepts multiple parameters:

```rust
use parameterized_test::create;

#[cfg(test)]
mod tests {
    use super::*;

    parameterized_test::create!{ commutative, (i, j, k), {
      assert_eq!(i, j);
      assert_eq!(j, k);
      assert_eq!(k, i); 
    }}
    commutative! {
        small: (1, 1, 1),
        large: (100, 100, 100),
    }
}
```
