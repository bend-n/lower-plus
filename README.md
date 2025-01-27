# lower

lowers expressions to their "desugared" form.

e.g

`a * b + c` => `(a.mul(b)).add(c)`

note that it is _extremely pervasive_, so

```rust
lower::math! { fn main() -> u8 {
    const X: u8 = 31 * 2;
    return 2 * X + 2;
} }
```

expands to

```rust
fn main() -> u8 {
    const X: u8 = 31.mul(2);
    return (2.mul(X)).add(2);
}
```

it should work for most expressions.

also implements some modules that let it work with some core intrinsics (`f*_fast`, `f*_algebraic`). (nightly only!)

## why

rust added an amazing feature called algebraic math (thanks orlp), allowing us to use `f*_algebraic`. however, dont you hate it when your

```rs
fn madd<const N: usize>(x: [[f32; 3]; N]) -> [f32; N] {
    x.map(|[a, b, c]| a * b + c) // not optimized! cant use `(v)fmadd`
}
```

turns into

```rs
fn madd<const N: usize>(x: [[f32; 3]; N]) -> [f32; N] {
    x.map(|[a, b, c]| core::intrinsics::fadd_algebraic(core::intrinsics::fmul_algebraic(a, b), c)) // readability in shambles
}
```

this crate allows you to

```rs
fn madd<const N: usize>(x: [[f32; 3]; N]) -> [f32; N] {
    // wow! such readability! ultimate simd!
    lower::algebraic! { x.map(|[a, b, c]| a * b + c) }
}
```
