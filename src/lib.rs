/*!
A macro for defining tests which accept arguments

# Example

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
*/

// Helper to escape the $ character in the nested macro, see
// https://github.com/rust-lang/rust/issues/35853#issuecomment-415993963
#[doc(hidden)]
#[macro_export]
macro_rules! __with_dollar_sign {
    ($($body:tt)*) => {
        macro_rules! __with_dollar_sign_ { $($body)* }
        __with_dollar_sign_!($);
    }
}

// Reexport anyhow so the macro can reference it from within clients' crates - see #1 and
// https://users.rust-lang.org/t/proc-macros-using-third-party-crate/42465/4
#[cfg(feature="propagation")]
pub use ::anyhow::Result as AnyhowResult;

// Duplicate the create!() macro to optionally support Result.
// https://stackoverflow.com/a/63011109/113632 suggests using another macro to reduce the amount of
// duplication, but this macro is messy enough already I think the redundancy is easier to deal with
#[cfg(feature="propagation")]
#[macro_export]
macro_rules! create {
    ($name:ident, $args:pat, $body:tt) => {
        $crate::__with_dollar_sign! {
            ($d:tt) => {
                macro_rules! $name {
                    ($d($d pname:ident: $d values:expr,)*) => {
                        mod $name {
                            #![ allow( unused_imports ) ]
                            use super::*;
                            $d(
                                #[test]
                                fn $d pname() -> $crate::AnyhowResult<()> {
                                    // TODO(https://github.com/rust-lang/rust/issues/69517)
                                    // although Rust 2018 supports Result-returning tests, the
                                    // failure behavior is very poor. This helper function f() can
                                    // be removed once Result tests are handled better. Demo:
                                    // https://play.rust-lang.org/?gist=b1a4d7bf42c885f42598d872877f2504
                                    fn f() -> $crate::AnyhowResult<()> {
                                        let $args = $d values;
                                        $body
                                        Ok(())
                                    }
                                    f().unwrap();
                                    Ok(())
                                }
                            )*
                        }}}}}}}

#[cfg(not(feature="propagation"))]
#[macro_export]
macro_rules! create {
    ($name:ident, $args:pat, $body:tt) => {
        $crate::__with_dollar_sign! {
            ($d:tt) => {
                macro_rules! $name {
                    ($d($d pname:ident: $d values:expr,)*) => {
                        mod $name {
                            #![ allow( unused_imports ) ]
                            use super::*;
                            $d(
                                #[test]
                                fn $d pname() {
                                    let $args = $d values;
                                    $body
                                }
                            )*
                        }}}}}}}

#[cfg(test)]
mod tests {
    create!{ basic, n, { assert!(n > 0); } }
    basic! {
        one: 1,
        two: 2,
        ten: 10,
    }

    create!{ multi_arg, (n, m), { assert!(n > m); } }
    multi_arg! {
        a: (10, 5),
        b: (4, 0),
        c: (1000, 500),
    }

    fn helper(arg: bool) -> bool { arg }

    create!{ calls_helper, arg, { assert!(helper(arg)); } }
    calls_helper! {
        arg: true,
    }

    #[cfg(feature="propagation")]
    create!{ propagation, n, { assert_eq!(n.to_string().parse::<i8>()? as i32, n); } }
    #[cfg(feature="propagation")]
    propagation! {
        a: 100,  // use a larger value, like 200, to see a parse error test failure
    }
}
