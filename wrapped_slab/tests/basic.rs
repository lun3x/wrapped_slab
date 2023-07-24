use wrapped_slab::WrappedSlab;

#[derive(WrappedSlab, PartialEq, Debug)]
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
    assert_eq!(next_entry_ref.0, format!("{next_key:?}"));

    let mut iter = slab.iter_mut();
    let (idx, s) = iter.next().unwrap();
    assert_eq!(idx, TestUnitStructKey(0));
    assert_eq!(s, &mut TestUnitStruct("TestUnitStructKey(0)".to_string()));
    s.0 = "modified".to_string();
    assert_eq!(iter.next(), None);

    let mut iter = slab.iter();
    let (idx, s) = iter.next().unwrap();
    assert_eq!(idx, TestUnitStructKey(0));
    assert_eq!(s, &TestUnitStruct("modified".to_string()));
    assert_eq!(iter.next(), None);

    let mut iter = slab.into_iter();
    let (idx, s) = iter.next().unwrap();
    assert_eq!(idx, TestUnitStructKey(0));
    assert_eq!(s, TestUnitStruct("modified".to_string()));
    assert_eq!(iter.next(), None);
}

#[derive(WrappedSlab, PartialEq, Debug)]
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
    assert_eq!(next_entry_ref.field1, format!("{next_key:?}"));

    let mut iter = slab.iter_mut();
    let (idx, s) = iter.next().unwrap();
    assert_eq!(idx, TestStructKey(0));
    assert_eq!(
        s,
        &mut TestStruct {
            field1: "TestStructKey(0)".to_string()
        }
    );
    s.field1 = "modified".to_string();
    assert_eq!(iter.next(), None);

    let mut iter = slab.iter();
    let (idx, s) = iter.next().unwrap();
    assert_eq!(idx, TestStructKey(0));
    assert_eq!(
        s,
        &TestStruct {
            field1: "modified".to_string()
        }
    );
    assert_eq!(iter.next(), None);

    let mut iter = slab.into_iter();
    let (idx, s) = iter.next().unwrap();
    assert_eq!(idx, TestStructKey(0));
    assert_eq!(
        s,
        TestStruct {
            field1: "modified".to_string()
        }
    );
    assert_eq!(iter.next(), None);
}

#[derive(WrappedSlab, PartialEq, Debug)]
enum TestEnum {
    VariantOne(String),
    VariantTwo,
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
    );

    let mut iter = slab.iter_mut();
    let (idx, s) = iter.next().unwrap();
    assert_eq!(idx, TestEnumKey(0));
    assert_eq!(s, &mut TestEnum::VariantOne("TestEnumKey(0)".to_string()));
    *s = TestEnum::VariantTwo;
    assert_eq!(iter.next(), None);

    let mut iter = slab.iter();
    let (idx, s) = iter.next().unwrap();
    assert_eq!(idx, TestEnumKey(0));
    assert_eq!(s, &TestEnum::VariantTwo);
    assert_eq!(iter.next(), None);

    let mut iter = slab.into_iter();
    let (idx, s) = iter.next().unwrap();
    assert_eq!(idx, TestEnumKey(0));
    assert_eq!(s, TestEnum::VariantTwo);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_drain() {
    let mut slab = TestEnumSlab::with_capacity(32);
    slab.reserve(64);
    assert_eq!(slab.capacity(), 64);

    slab.insert(TestEnum::VariantTwo);

    let mut drain = slab.drain();
    assert_eq!(drain.len(), 1);
    let s = drain.next().unwrap();
    assert_eq!(s, TestEnum::VariantTwo);
    assert_eq!(drain.len(), 0);
    drop(drain);

    assert_eq!(slab.len(), 0);
}
