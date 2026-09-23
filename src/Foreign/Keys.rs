// `Foreign.Keys` FFI: the own property names of a foreign object or record.
pub fn Foreign_Keys_unsafeKeys(value: crate::UnknownType) -> crate::UnknownType {
    let entries = value
        .__purust_foreign_object()
        .entries()
        .into_iter()
        .map(|(name, _)| crate::Value::String(name))
        .collect();
    crate::mk_array(entries)
}
