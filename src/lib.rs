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
}
