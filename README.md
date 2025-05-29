A an alternative to `Range<T>` that has a defined memory layout, implements
[`std::marker::Copy`], and has some convenience methods.

```rust
use copyspan::Span;

let text = "hello world";
let s = Span::from(6..11);

for i in s {
    dbg!(i);
}

// Because `Span` is copyable, we can reuse it without calling `clone`
assert_eq!(&text[s], "world");
assert_eq!(&text[s.with_len(2)], "wo");
```

This is also useful for making copyable datastructures that contain ranges.
```rust
use copyspan::Span;
use std::ops::Range;

#[derive(Clone, Copy, Default)]
struct HoldsSpan {
    x: Span<usize>,
}

fn expects_range(_: Range<usize>) {}
fn takes_val(_: HoldsSpan) {}

let val = HoldsSpan::default();
takes_val(val); // If `HoldSpan` wasn't `Copy`, `val` would be moved into this function

expects_range(val.x.range());
```
