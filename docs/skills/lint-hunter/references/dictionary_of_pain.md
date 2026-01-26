# Lint Hunter: Dictionary of Pain

### E0502: Cannot borrow `x` as mutable because it is also borrowed as immutable

- **Cause**: You have a reference `&x` active (an "immutable loan") and you try to do `&mut x` or modify `x`.
- **Fix 1 (Re-scoping)**: Use a block `{}` to force the immutable borrow to drop before the mutation.
- **Fix 2 (Cloning)**: If you need the value for the mutation, `clone()` the data _before_ taking the mutable borrow.
  - _Example_: `let val = x[0].clone(); x.push(val);`

### E0382: Use of moved value

- **Cause**: You passed `x` (by value) to a function or loop, and then tried to use it again.
- **Fix 1 (Reference)**: Pass `&x` if ownership transfer isn't needed.
- **Fix 2 (Clone)**: If the consumer needs ownership, `x.clone()`.
- **Fix 3 (Derive Copy)**: If the type is trivial (primitive), ensure it derives `Copy`.

### E0597: `x` does not live long enough

- **Cause**: You are returning a reference to a local variable that is about to be dropped.
- **Fix 1 (Ownership)**: Return the `String` (owned), not `&str` (borrowed).
- **Fix 2 (Lifetime Lifting)**: Declare the storage variable _outside_ the scope where the reference is taken.
