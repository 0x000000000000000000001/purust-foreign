pub fn Foreign_tagOf(value: crate::UnknownType) -> String {
    match value.resolve() {
        crate::Value::Unit => "Undefined",
        crate::Value::Null => "Null",
        crate::Value::Int(_) | crate::Value::Number(_) => "Number",
        crate::Value::String(_) | crate::Value::Char(_) => "String",
        crate::Value::Bool(_) => "Boolean",
        crate::Value::Array(_) => "Array",
        // JS.BigInt re-exports this exact pinned native type. Depend on its
        // carrier crate, not Purs_JS_BigInt, to keep Foreign usable on its own.
        crate::Value::Class(native)
            if native.is::<std::rc::Rc<num_bigint_dig::BigInt>>() => "BigInt",
        crate::Value::Class(_) => "Object",
        _ if value.__purust_record_fields().is_some() => "Object",
        _ => "Function",
    }.to_owned()
}

pub fn Foreign_isNull(value: crate::UnknownType) -> bool {
    matches!(value.resolve(), purust_core::Value::Null)
}

pub fn Foreign_isUndefined(value: crate::UnknownType) -> bool {
    matches!(value.resolve(), purust_core::Value::Unit)
}

pub fn Foreign_isArray(value: crate::UnknownType) -> bool {
    matches!(value.resolve(), purust_core::Value::Array(_))
}

pub fn Foreign_typeOf(value: crate::UnknownType) -> String {
    match Foreign_tagOf(value).as_str() {
        "Undefined" => "undefined",
        "Number" => "number",
        "BigInt" => "bigint",
        "String" => "string",
        "Boolean" => "boolean",
        "Function" => "function",
        _ => "object",
    }.to_owned()
}
