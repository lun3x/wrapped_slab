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
}
