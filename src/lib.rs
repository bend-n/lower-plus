//! a lil macro crate.
//!
//! provides a handy macro for converting `a + b` to `a.add(b)` for when you cant easily overload the `Add` trait,
//! and some traits that provide [`f*_algebraic`](algebraic::math) and [`f*_fast`](fast::math) implementations. (requires nightly)
#![allow(internal_features)]
#![cfg_attr(
    feature = "modules",
    feature(doc_auto_cfg, core_intrinsics, hint_assert_unchecked)
)]

#[cfg(feature = "modules")]
pub mod algebraic;
#[cfg(feature = "modules")]
pub mod fast;

/// Lower math to method calls. Only useful if you define the functions.
/// ```
/// # use std::ops::*;
/// let [a, b, c] = [5i32, 6, 7];
/// assert_eq!(lower::math! { a * *&b + -c }, a * *&b + -c);
/// // expands to
/// // a.mul((&b).deref()).add(c.neg())
/// ```
#[macro_export]
macro_rules! math {
    ($($x:tt)+) => {
        lower_macros::math! { $($x)+ }
    };
}
