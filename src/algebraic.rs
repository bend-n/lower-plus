//! Provides the algebraic math trait and macro.
/// Trait for Algebraic-ly math'd floats. Try to use [`algebraic::math`](`math`).
pub trait Float: Copy {
    fn add(self, other: Self) -> Self;
    fn sub(self, other: Self) -> Self;
    fn mul(self, other: Self) -> Self;
    fn div(self, other: Self) -> Self;
    fn rem(self, other: Self) -> Self;

    fn eq(self, other: Self) -> bool;
    fn lt(self, other: Self) -> bool;
    fn le(self, other: Self) -> bool;
    fn ne(self, other: Self) -> bool;
    fn ge(self, other: Self) -> bool;
    fn gt(self, other: Self) -> bool;

    fn deref(&self) -> Self {
        *self
    }
    fn neg(self) -> Self;
}
use std::intrinsics::*;
macro_rules! imp {
    ($this:ty) => {
#[rustfmt::skip]
impl Float for $this {
    fn add(self, other: Self) -> Self { fadd_algebraic(self, other) }
    fn sub(self, other: Self) -> Self { fsub_algebraic(self, other) }
    fn mul(self, other: Self) -> Self { fmul_algebraic(self, other) }
    fn div(self, other: Self) -> Self { fdiv_algebraic(self, other) }
    fn rem(self, other: Self) -> Self { frem_algebraic(self, other) }

    fn eq(self, other: Self) -> bool { self == other }
    fn lt(self, other: Self) -> bool { self < other }
    fn le(self, other: Self) -> bool { self <= other }
    fn ne(self, other: Self) -> bool { self != other }
    fn ge(self, other: Self) -> bool { self >= other }
    fn gt(self, other: Self) -> bool { self > other }

    fn deref(&self) -> Self { *self }
    fn neg(self) -> Self { -self }
}
    };
}
imp!(f32);
imp!(f64);

/// Changes all the math to algebraic float math. See [`fadd_algebraic`].
///
/// This allows automatic vectorization of float math easier for the compiler, among other things.
/// If you intend to create a function in this, first import [`lower::algebraic::Float`](crate::algebraic::Float), use [`lower::math`](crate::math).
///
/// This would lower `a + b + c + d` to `alge_add(alge_add(alge_add(a, b), c), d)` (which could be organized by the compiler into `(a + b) + (c + d)` if it chooses)
#[macro_export]
#[doc(hidden)]
macro_rules! algebraic {
    ($($x:tt)+) => {{
        use $crate::algebraic::Float as _;
        lower_macros::math! { $($x)+ }
    }}
}
#[doc(inline)]
pub use algebraic as math;
