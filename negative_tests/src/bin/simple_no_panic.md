```rust
#[::no_panic::no_panic]
fn divide() {
     core::hint::black_box(1 / core::hint::black_box(1));
}

#[::no_panic::no_panic]
fn slice_access() {
    const A: [bool; 2] = [true, false];
    let s = core::hint::black_box( &A[..] );
    let s = core::hint::black_box( s );
}

divide();
slice_access();
```
