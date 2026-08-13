## sure

Refinement types that track their set of possible values and propagate them through operations.\
Implemented using const generics at no extra runtime cost.

This crate makes use of several unstable features, so it can currently only be used on a nightly toolchain.

## Example
```rust
use sure::SureU8;
use sure::sure_u8::Range;
fn main() {
    // `foo` is known to be in the set {1, 2, 3}
    let foo: SureU8![1, 2, 3] = SureU8::new(2).unwrap();

    // `bar` is known to be in the set {5, 6}
    let bar: SureU8![5, 6] = SureU8::new(6).unwrap();

    // `baz` is known to be in the set {5, 10, 15, 6, 12, 18},
    // precisely because those are the only possible results
    // from multiplying an element from `foo`'s set with one from `bar`'s set.
    let baz: SureU8![5, 6, 10, 12, 15, 18] = foo * bar;

    assert_eq!(baz.inner(), 12);

    let a: SureU8![Range![4..=100]] = SureU8::new(20).unwrap();
    let b: SureU8![Range![10..=20]] = SureU8::new(12).unwrap();

    // `normalize` is a no-op on the stored value
    // the only thing it does is sort and de-duplicate the numbers stored in the type.
    let c: SureU8![Range![14..=120]] = a.strict_add(b).normalize();

    assert_eq!(c.inner(), 32);
}
```
