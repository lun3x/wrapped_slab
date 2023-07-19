use wrapped_slab::WrappedSlab;

#[derive(WrappedSlab)]
struct TestUnitStruct(String);

#[test]
fn test_unit_struct() {
    let mut slab = TestUnitStructSlab::with_capacity(32);
    slab.reserve(64);
    assert_eq!(slab.capacity(), 64);

    let key: TestUnitStructKey = slab.insert(TestUnitStruct("testing".into()));

    let val: Option<&TestUnitStruct> = slab.get(key);
    assert_eq!(val.unwrap().0, "testing");

    let val = slab.remove(key);
    assert_eq!(val.0, "testing");

    assert_eq!(slab.len(), 0);

    let next_entry: TestUnitStructVacantEntry = slab.vacant_entry();
    let next_key: TestUnitStructKey = next_entry.key();
    let next_entry_ref: &mut TestUnitStruct =
        next_entry.insert(TestUnitStruct(format!("{next_key:?}")));
    assert_eq!(next_entry_ref.0, format!("{next_key:?}"))
}

#[derive(WrappedSlab)]
struct TestStruct {
    field1: String,
}

#[test]
fn test_struct() {
    let mut slab = TestStructSlab::with_capacity(32);
    slab.reserve(64);
    assert_eq!(slab.capacity(), 64);

    let key: TestStructKey = slab.insert(TestStruct {
        field1: "testing".into(),
    });

    let val: Option<&TestStruct> = slab.get(key);
    assert_eq!(val.unwrap().field1, "testing");

    let val = slab.remove(key);
    assert_eq!(val.field1, "testing");

    assert_eq!(slab.len(), 0);

    let next_entry: TestStructVacantEntry = slab.vacant_entry();
    let next_key: TestStructKey = next_entry.key();
    let next_entry_ref: &mut TestStruct = next_entry.insert(TestStruct {
        field1: format!("{next_key:?}"),
    });
    assert_eq!(next_entry_ref.field1, format!("{next_key:?}"))
}

#[derive(WrappedSlab, PartialEq, Debug)]
enum TestEnum {
    VariantOne(String),
}

#[test]
fn test_enum() {
    let mut slab = TestEnumSlab::with_capacity(32);
    slab.reserve(64);
    assert_eq!(slab.capacity(), 64);

    let key: TestEnumKey = slab.insert(TestEnum::VariantOne("testing".into()));

    let val: Option<&TestEnum> = slab.get(key);
    assert_eq!(val, Some(&TestEnum::VariantOne("testing".into())));

    let val = slab.remove(key);
    assert_eq!(val, TestEnum::VariantOne("testing".into()));

    assert_eq!(slab.len(), 0);

    let next_entry: TestEnumVacantEntry = slab.vacant_entry();
    let next_key: TestEnumKey = next_entry.key();
    let next_entry_ref: &mut TestEnum =
        next_entry.insert(TestEnum::VariantOne(format!("{next_key:?}")));
    assert_eq!(
        next_entry_ref,
        &mut TestEnum::VariantOne(format!("{next_key:?}"))
    )
}
