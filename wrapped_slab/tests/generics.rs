use wrapped_slab::WrappedSlab;

#[derive(WrappedSlab, PartialEq)]
struct TestUnitStruct<A>(A);

#[test]
fn test_unit_struct() {
    let mut slab: TestUnitStructSlab<&str> = TestUnitStructSlab::<&str>::with_capacity(32);
    slab.reserve(64);
    assert_eq!(slab.capacity(), 64);

    let key: TestUnitStructKey = slab.insert(TestUnitStruct("testing"));

    let val: Option<&TestUnitStruct<&str>> = slab.get(key);
    assert_eq!(val.unwrap().0, "testing");

    let val = slab.remove(key);
    assert_eq!(val.0, "testing");

    assert_eq!(slab.len(), 0);
}

#[derive(WrappedSlab, PartialEq)]
struct TestUnitConstStruct<const N: usize>([f32; N]);

#[test]
fn test_unit_const_struct() {
    let mut slab: TestUnitConstStructSlab<1> = TestUnitConstStructSlab::with_capacity(32);
    slab.reserve(64);
    assert_eq!(slab.capacity(), 64);

    let key: TestUnitConstStructKey = slab.insert(TestUnitConstStruct([42.0]));

    let val: Option<&TestUnitConstStruct<1>> = slab.get(key);
    assert_eq!(val.unwrap().0, [42.0]);

    let val = slab.remove(key);
    assert_eq!(val.0, [42.0]);

    assert_eq!(slab.len(), 0);
}

#[derive(WrappedSlab, PartialEq)]
struct TestUnitParamStruct<A: Default>(A);

#[test]
fn test_unit_param_struct() {
    let mut slab: TestUnitParamStructSlab<&str> =
        TestUnitParamStructSlab::<&str>::with_capacity(32);
    slab.reserve(64);
    assert_eq!(slab.capacity(), 64);

    let key: TestUnitParamStructKey = slab.insert(TestUnitParamStruct("testing"));

    let val: Option<&TestUnitParamStruct<&str>> = slab.get(key);
    assert_eq!(val.unwrap().0, "testing");

    let val = slab.remove(key);
    assert_eq!(val.0, "testing");

    assert_eq!(slab.len(), 0);
}

#[derive(WrappedSlab, PartialEq)]
struct TestTupleStruct<A, B>((A, B));

#[test]
fn test_tuple_struct() {
    let mut slab: TestTupleStructSlab<&str, f64> =
        TestTupleStructSlab::<&str, f64>::with_capacity(32);
    slab.reserve(64);
    assert_eq!(slab.capacity(), 64);

    let key: TestTupleStructKey = slab.insert(TestTupleStruct(("testing", 42.0)));

    let val: Option<&TestTupleStruct<&str, f64>> = slab.get(key);
    assert_eq!(val.unwrap().0, ("testing", 42.0));

    let val = slab.remove(key);
    assert_eq!(val.0, ("testing", 42.0));

    assert_eq!(slab.len(), 0);
}

#[derive(WrappedSlab, PartialEq)]
struct TestTupleConstStruct<A, const N: usize>((A, [f32; N]));

#[test]
fn test_tuple_const_struct() {
    let mut slab = TestTupleConstStructSlab::<&str, 1>::with_capacity(32);
    slab.reserve(64);
    assert_eq!(slab.capacity(), 64);

    let key: TestTupleConstStructKey = slab.insert(TestTupleConstStruct(("testing", [42.0])));

    let val: Option<&TestTupleConstStruct<&str, 1>> = slab.get(key);
    assert_eq!(val.unwrap().0, ("testing", [42.0]));

    let val = slab.remove(key);
    assert_eq!(val.0, ("testing", [42.0]));

    assert_eq!(slab.len(), 0);
}

#[derive(WrappedSlab, PartialEq)]
struct TestStruct<A> {
    field1: A,
}

#[test]
fn test_struct() {
    let mut slab: TestStructSlab<&str> = TestStructSlab::<&str>::with_capacity(32);
    slab.reserve(64);
    assert_eq!(slab.capacity(), 64);

    let key: TestStructKey = slab.insert(TestStruct { field1: "testing" });

    let val: Option<&TestStruct<&str>> = slab.get(key);
    assert_eq!(val.unwrap().field1, "testing");

    let val = slab.remove(key);
    assert_eq!(val.field1, "testing");

    assert_eq!(slab.len(), 0);
}

#[derive(WrappedSlab, PartialEq, Debug)]
enum TestEnum<A> {
    VariantOne(A),
    VariantTwo,
}

#[test]
fn test_enum() {
    let mut slab = TestEnumSlab::with_capacity(32);
    slab.reserve(64);
    assert_eq!(slab.capacity(), 64);

    let key: TestEnumKey = slab.insert(TestEnum::VariantOne("testing"));

    let val: Option<&TestEnum<&str>> = slab.get(key);
    assert_eq!(val, Some(&TestEnum::VariantOne("testing")));

    let val = slab.remove(key);
    assert_eq!(val, TestEnum::VariantOne("testing"));

    assert_eq!(slab.len(), 0);
}

#[test]
fn test_vacant_entry() {
    let mut slab = TestEnumSlab::default();
    slab.insert(TestEnum::VariantOne("testing".into()));

    let next_entry: TestEnumVacantEntry<String> = slab.vacant_entry();
    let next_key: TestEnumKey = next_entry.key();
    let next_entry_ref: &mut TestEnum<String> =
        next_entry.insert(TestEnum::VariantOne(format!("{next_key}")));
    assert_eq!(
        next_entry_ref,
        &mut TestEnum::VariantOne(format!("{next_key}"))
    );
}

#[test]
fn test_iter() {
    let mut slab = TestEnumSlab::default();
    slab.insert(TestEnum::VariantOne("testing".into()));

    let mut iter = slab.iter_mut();
    let (idx, s) = iter.next().unwrap();
    assert_eq!(idx, TestEnumKey(0));
    assert_eq!(s, &mut TestEnum::VariantOne("testing".to_string()));
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
    let mut slab = TestEnumSlab::<String>::with_capacity(32);
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

#[test]
fn test_index() {
    let mut slab = TestEnumSlab::<String>::with_capacity(32);
    slab.reserve(64);
    assert_eq!(slab.capacity(), 64);

    slab.insert(TestEnum::VariantTwo);

    let s = &slab[TestEnumKey(0)];
    assert_eq!(s, &TestEnum::VariantTwo);

    let s = &mut slab[TestEnumKey(0)];
    assert_eq!(s, &mut TestEnum::VariantTwo);
}
