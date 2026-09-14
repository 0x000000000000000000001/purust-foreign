use std::rc::Rc;

fn purust_foreign_property(key: crate::UnknownType, value: &crate::UnknownType) -> crate::UnknownType {
    let key = match key.resolve() {
        crate::Value::String(key) => key.clone(),
        crate::Value::Int(index) => index.to_string(),
        _ => panic!("Foreign.Index: unsupported property key"),
    };
    let index = key.parse::<usize>().ok().filter(|index| index.to_string() == key);
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
