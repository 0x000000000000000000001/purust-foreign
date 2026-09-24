use std::rc::Rc;

fn property_key(key: crate::UnknownType) -> String {
    match key.resolve() {
        crate::Value::String(key) => key.clone(),
        crate::Value::Int(index) => index.to_string(),
        _ => panic!("Foreign.Index: unsupported property key"),
    }
}

/// A decimal element index only; `"01"`, `"+1"` and `"1.0"` are property
/// names, not elements, like JavaScript's array index rules.
fn array_index(key: &str) -> Option<usize> {
    key.parse::<usize>().ok().filter(|index| index.to_string() == key)
}

fn purust_foreign_property(key: crate::UnknownType, value: &crate::UnknownType) -> crate::UnknownType {
    let key = property_key(key);
    let index = array_index(&key);
    let property = match value.resolve() {
        crate::Value::Array(items) if key == "length" => Some(crate::mk_int(items.len() as i64)),
        crate::Value::Array(items) => index.and_then(|i| items.get(i).cloned()),
        crate::Value::String(text) if key == "length" => Some(crate::mk_int(text.chars().count() as i64)),
        crate::Value::String(text) => index.and_then(|i| text.chars().nth(i)).map(|c| crate::mk_string(&c.to_string())),
        crate::Value::Class(native) => native.downcast_ref::<Rc<purust_core::SharedRecord>>()
            .expect("Foreign.Index: unsupported opaque native object").get(&key),
        _ => value.__purust_record_fields().and_then(|fields| fields.get(&key).cloned()),
    };
    // JS undefined has the same representation as PureScript Unit.
    property.unwrap_or(crate::Value::Unit)
}

pub fn Foreign_Index_unsafeReadPropImpl() -> crate::UnknownType {
    crate::Value::Func4(purust_core::Func4::Static(|failure, success, key, value| {
        if matches!(value.resolve(), crate::Value::Unit | crate::Value::Null) {
            failure
        } else {
            success.unwrap_func1()(purust_foreign_property(key, &value))
        }
    }))
}

/// Own-property test over native carriers: record fields, array elements and
/// `"length"`, and `Foreign.Object` handles. JavaScript prototype properties
/// (`"map" in []`) have no native counterpart.
fn has_own_property(key: crate::UnknownType, value: &crate::UnknownType) -> bool {
    let key = property_key(key);
    match value.resolve() {
        crate::Value::Array(items) => {
            key == "length" || array_index(&key).map_or(false, |index| index < items.len())
        }
        crate::Value::String(text) => {
            key == "length" || array_index(&key).map_or(false, |index| index < text.chars().count())
        }
        crate::Value::Class(native) => native
            .downcast_ref::<Rc<purust_core::SharedRecord>>()
            .map_or(false, |record| record.get(&key).is_some()),
        _ => value
            .__purust_record_fields()
            .map_or(false, |fields| fields.get(&key).is_some()),
    }
}

pub fn Foreign_Index_unsafeHasOwnProperty() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Static(|key, value| {
        crate::mk_bool(has_own_property(key, &value))
    }))
}

// Without a prototype chain, `hasProperty` keeps the own-property contract
// instead of pretending prototype names exist.
pub fn Foreign_Index_unsafeHasProperty() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Static(|key, value| {
        crate::mk_bool(has_own_property(key, &value))
    }))
}
