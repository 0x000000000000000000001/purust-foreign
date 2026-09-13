pub fn Foreign_tagOf(value: crate::UnknownType) -> String {
    match value.resolve() {
        crate::Value::Unit => "Undefined",
        crate::Value::Int(_) | crate::Value::Number(_) => "Number",
        crate::Value::String(_) | crate::Value::Char(_) => "String",
        crate::Value::Bool(_) => "Boolean",
        crate::Value::Array(_) => "Array",
        crate::Value::Class(_) => "Object",
        _ if value.__purust_record_fields().is_some() => "Object",
        _ => "Function",
    }.to_owned()
}

pub fn Foreign_typeOf(value: crate::UnknownType) -> String {
    match Foreign_tagOf(value).as_str() {
        "Undefined" => "undefined",
        "Number" => "number",
        "String" => "string",
        "Boolean" => "boolean",
        "Function" => "function",
        _ => "object",
    }.to_owned()
}
