//! a lil macro crate.
//!
//! provides a handy macro for converting `a + b` to `a.add(b)` for when you cant easily overload the `Add` trait,
//! and some traits that provide [`f*_algebraic`](algebraic::math) and [`f*_fast`](fast::math) implementations. (requires nightly)
/// Provides the algebraic math trait and macro.
pub mod algebraic {
    /// Changes all the math to algebraic float math. See [`fadd_algebraic`](core::intrinsics::fadd_algebraic).
    ///
    /// This allows automatic vectorization of float math easier for the compiler, among other things.
    ///
    /// This would lower `a + b + c + d` to `alge_add(alge_add(alge_add(a, b), c), d)` (which could be organized by the compiler into `(a + b) + (c + d)` if it chooses)
    pub use lower_macros::algebraic as math;
}
/// Provides the fast math trait and macro. See terms and conditions[^1].
///
/// [^1]: <https://simonbyrne.github.io/notes/fastmath>
pub mod fast {
    /// Changes all the math to fast float math. See [`fadd_fast`](core::intrinsics::fadd_fast).
    ///
    /// This allows automatic vectorization of float math for the compiler, among other things.
    ///
    /// This would lower `a * b + c` to `fastadd(fastmul(a, b), c)`.
    pub use lower_macros::fast as math;
}

/// Lower math to method calls. Only useful if you define the functions.
/// ```
/// # use std::ops::*;
/// let [a, b, c] = [5i32, 6, 7];
/// assert_eq!(lower::math! { a * *&b + -c }, a * *&b + -c);
/// // expands to
/// // a.mul((&b).deref()).add(c.neg())
/// ```
/// <p style="display: none">
pub use lower_macros::math;
