//! Provides the fast math trait and macro. See terms and conditions[^1].
//!
//! [^1]: <https://simonbyrne.github.io/notes/fastmath>
#[allow(unused_imports)]
use std::f32::{INFINITY as INF, NAN};

macro_rules! s {
    ($($x:tt)+) => {
        #[doc = include_str!("fast.safety.md")]
        $($x)+;
    };
}

#[rustfmt::skip]
/// Trait for fast math floats. Try to use [`fast::math`](crate::fast::math).
/// These functions all assume that your float is NOT [`INF`] | [`NAN`].
/// If so undefined behaviour is incurred.
pub unsafe trait Float: Copy {
    s!(unsafe fn add(self, other: Self) -> Self);
    s!(unsafe fn sub(self, other: Self) -> Self);
    s!(unsafe fn mul(self, other: Self) -> Self);
    s!(unsafe fn div(self, other: Self) -> Self);
    s!(unsafe fn rem(self, other: Self) -> Self);

    s!(unsafe fn eq(self, other: Self) -> bool);
    s!(unsafe fn lt(self, other: Self) -> bool);
    s!(unsafe fn le(self, other: Self) -> bool);
    s!(unsafe fn ne(self, other: Self) -> bool);
    s!(unsafe fn ge(self, other: Self) -> bool);
    s!(unsafe fn gt(self, other: Self) -> bool);

    fn deref(&self) -> Self {
        *self
    }
    fn neg(self) -> Self;

    #[doc(hidden)]
    unsafe fn finite(self, other: Self) -> Self;
}

use std::intrinsics::*;
macro_rules! imp {
    ($this:ty) => {
#[rustfmt::skip]
unsafe impl Float for $this {
    #[inline(always)]
    unsafe fn finite(self, other: Self) -> Self {
        core::hint::assert_unchecked(self.is_finite());
        core::hint::assert_unchecked(other.is_finite());
        self
    }
    unsafe fn add(self, other: Self) -> Self { fadd_fast(self.finite(other), other) }
    unsafe fn sub(self, other: Self) -> Self { fsub_fast(self.finite(other), other) }
    unsafe fn mul(self, other: Self) -> Self { fmul_fast(self.finite(other), other) }
    unsafe fn div(self, other: Self) -> Self { fdiv_fast(self.finite(other), other) }
    unsafe fn rem(self, other: Self) -> Self { frem_fast(self.finite(other), other) }

    unsafe fn eq(self, other: Self) -> bool { (self.finite(other) + 0.0).to_bits() == (other + 0.0).to_bits() }
    unsafe fn lt(self, other: Self) -> bool { self.finite(other) < other }
    unsafe fn le(self, other: Self) -> bool { self.finite(other) <= other }
    unsafe fn ne(self, other: Self) -> bool { self.finite(other) != other }
    unsafe fn ge(self, other: Self) -> bool { self.finite(other) >= other }
    unsafe fn gt(self, other: Self) -> bool { self.finite(other) > other }

    fn deref(&self) -> Self { *self }
    fn neg(self) -> Self { -self }
}
    };
}
imp!(f32);
imp!(f64);

/// Changes all the math to fast float math. See [`fadd_fast`].
///
/// This allows automatic vectorization of float math for the compiler, among other things.
/// If you intend to create a function in this, first import [`lower::fast::Float`](crate::fast::Float), use [`lower::math`](crate::math).
///
/// This would lower `a * b + c` to `fadd(fmul(a, b), c)`.
#[macro_export]
#[doc(hidden)]
macro_rules! fast {
    ($($x:tt)+) => {{
        use $crate::fast::Float as _;
        lower_macros::math! { $($x)+ }
    }}
}
#[doc(inline)]
pub use fast as math;
