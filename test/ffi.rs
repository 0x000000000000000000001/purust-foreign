//! FFI classification and own-property contracts on native carriers.
//!
//! Mirrors the Go runtime's `function-data_test.go` where a native counterpart
//! exists: ordinary functions and partially applied closures keep their
//! JavaScript classification when they cross the Foreign boundary. The Go
//! wrapper metadata (`WithFunctionData`) and interface boxing have no Rust
//! equivalent and are intentionally absent.
use purust_core::*;
use std::rc::Rc;

#[test]
fn function_carriers_classify_as_functions() {
    use Purs_Foreign::*;

    let ordinary = Value::Func2(Func2::Static(|_, _| Value::Unit));
    let captured = mk_int(7);
    let partial = Value::Func1(Func1::Shared(Rc::new(move |_| captured.clone())));
    for value in [ordinary, partial] {
        assert_eq!(Foreign_typeOf(value.clone()), "function");
        assert_eq!(Foreign_tagOf(value), "Function");
    }
}

#[test]
fn object_and_array_carriers_classify_like_javascript() {
    use Purs_Foreign::*;

    let record = record("name", "purescript");
    let object = Value::Class(Rc::new(record));
    assert_eq!(Foreign_typeOf(object.clone()), "object");
    assert_eq!(Foreign_tagOf(object), "Object");

    let array = mk_array(vec![mk_int(1), mk_int(2), mk_int(3)]);
    assert_eq!(Foreign_typeOf(array.clone()), "object");
    assert_eq!(Foreign_tagOf(array.clone()), "Array");
    assert!(Foreign_isArray(array.clone()));
    assert!(!Foreign_isArray(mk_int(42)));
}

// `Foreign.Object` handles cross the boundary as `Class(Rc<Rc<SharedRecord>>)`,
// the shape the generated code produces and the readers downcast.
fn record(name: &str, value: &str) -> Rc<SharedRecord> {
    Rc::new(SharedRecord::from_entries(vec![(
        name.to_owned(),
        mk_string(value),
    )]))
}

#[test]
fn own_property_ffi_covers_records_arrays_and_handles() {
    use Purs_Foreign_Index::*;

    let has_own = Foreign_Index_unsafeHasOwnProperty().unwrap_func2();
    let has_property = Foreign_Index_unsafeHasProperty().unwrap_func2();

    let record = Value::Class(Rc::new(record("name", "purescript")));
    assert!(has_own(mk_string("name"), record.clone()).unwrap_bool());
    assert!(!has_own(mk_string("missing"), record.clone()).unwrap_bool());
    assert!(has_property(mk_string("name"), record.clone()).unwrap_bool());
    assert!(!has_property(mk_string("missing"), record).unwrap_bool());

    let array = mk_array(vec![mk_int(10), mk_int(20)]);
    assert!(has_own(mk_string("length"), array.clone()).unwrap_bool());
    assert!(has_own(mk_int(1), array.clone()).unwrap_bool());
    assert!(!has_own(mk_int(2), array.clone()).unwrap_bool());
    // `"01"`, `"+1"` and `"1.0"` are property names, not element indices.
    assert!(!has_own(mk_string("01"), array.clone()).unwrap_bool());
    assert!(!has_own(mk_string("1.0"), array).unwrap_bool());
}
