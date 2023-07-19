# WrappedSlab [![Tests](https://github.com/lun3x/wrapped_slab/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/lun3x/wrapped_slab/actions/workflows/rust.yml)

[Also available on crates.io.](https://crates.io/crates/wrapped_slab)

Very simple Rust library useful when you want stronger type guarantees than Slab's usize keys. Generates `TSlab(Slab<T>)` that accepts `TKey` instead of `usize`. `TVacantEntry(VacantEntry<T>)` is also generated along the same lines.

## Example
```rust
use wrapped_slab::WrappedSlab;

#[derive(WrappedSlab)]
struct TestUnitStruct(String);

fn main() {
    let mut slab = TestUnitStructSlab::default();
    let key: TestUnitStructKey = slab.insert(TestUnitStruct("testing".into()));
    let val: Option<&TestUnitStruct> = slab.get(key);
    let next_entry: TestUnitStructVacantEntry = slab.vacant_entry();
    let next_key: TestUnitStructKey = next_entry.key();
    let next_entry_ref: &mut TestUnitStruct = next_entry.insert(TestUnitStruct(format!("{next_key:?}")));

    // See wrapped_slab/tests/ for more examples
}
```