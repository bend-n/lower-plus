//! a lil macro crate.
//!
//! provides a handy macro for converting `a + b` to `a.add(b)` for when you cant easily overload the [`Add`](core::ops::Add) trait,
//! and some macros that provide [`f*_algebraic`](algebraic::math) (requires nightly), [`f*_fast`](fast::math) (requires nightly), [`saturating_*`](u8::saturating_add), and [`wrapping_*`](u8::wrapping_sub) implementations.
#![no_std]
/// Provides the algebraic math macro.
pub mod algebraic {
    /// Changes all the math to algebraic float math. See [`fadd_algebraic`](core::intrinsics::fadd_algebraic).
    ///
    /// This allows automatic vectorization of float math easier for the compiler, among other things.
    ///
    /// This would lower `a + b + c + d` to `alge_add(alge_add(alge_add(a, b), c), d)` (which could be organized by the compiler into `(a + b) + (c + d)` if it chooses)
    pub use lower_macros::algebraic as math;
}
/// Provides the fast math macro. See terms and conditions[^1].
/// Prefer [`algebraic`] in most cases.
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

/// Provides the saturating math macro.
pub mod saturating {
    /// Changes all the math to saturating math. See [`core::num::Saturating`]
    ///
    /// This would lower `a * b + c` to `((a).saturating_mul(b)).saturating_add(c)`.
    pub use lower_macros::saturating as math;
}

/// Provides the wrapping math macro.
pub mod wrapping {
    /// Changes all the math to saturating math. See [`core::num::Wrapping`]
    ///
    /// This would lower `a * b + c` to `((a).wrapping_mul(b)).wrapping_add(c)`.
    pub use lower_macros::wrapping as math;
}

/// Applys the macro to your stmts.
/// Argument can be `algebraic` | `fast` | `basic` | `wrapping` | `saturating` | none
/// ```
/// # #![feature(core_intrinsics)]
/// #[lower::apply(algebraic)]
/// fn madd(a: f32, b: f32, c: f32) -> f32 {
///   a * b + c
/// }
/// ```
/// Enaables nicer formatting and better rust analyzer integration.
pub use lower_macros::apply;

/// Lower math to method calls. Only useful if you define the functions.
/// ```
/// # use std::ops::*;
/// let [a, b, c] = [5i32, 6, 7];
/// assert_eq!(lower::math! { a * *&b + -c }, a * *&b + -c);
/// // expands to
/// // a.mul((&b).deref()).add(c.neg())
/// ```
pub use lower_macros::math;
