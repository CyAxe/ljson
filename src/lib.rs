use mlua::{Error, ExternalError, Function, Lua, LuaSerdeExt, Nil, Result, Table, Value};
use serde::Serialize;

/// Decodes input value (must be a string) that represents a yaml document to a Lua value
fn decode<'lua>(lua: &'lua Lua, s: Value<'lua>) -> Result<Value<'lua>> {
    let s = match s {
        Value::String(ref s) => Ok(s.as_bytes()),
        _ => Err(format!("invalid input type: {}", s.type_name()).to_lua_err()),
    }?;
    let json_value = serde_json::from_slice::<serde_json::Value>(s)
        .map_err(|e| Error::external(e.to_string()))?;
    lua.to_value(&json_value)
}

/// Encodes Lua value (any supported) to a yaml document
fn encode<'lua>(lua: &'lua Lua, v: Value<'lua>) -> Result<Value<'lua>> {
    let mut buf = Vec::new();
    v.serialize(&mut serde_json::Serializer::new(&mut buf))
        .map_err(|e| Error::external(e.to_string()))?;
    lua.create_string(&buf).map(Value::String)
}

fn make_exports<'lua>(
    lua: &'lua Lua,
    decode: Function<'lua>,
    encode: Function<'lua>,
) -> Result<Table<'lua>> {
    let exports = lua.create_table().unwrap();
    exports.set("load", decode.clone()).unwrap();
    exports.set("decode", decode).unwrap();
    exports.set("dump", encode.clone()).unwrap();
    exports.set("encode", encode).unwrap();
    exports.set("null", lua.null()).unwrap();
    exports.set("array_mt", lua.array_metatable()).unwrap();
    Ok(exports)
}

#[mlua::lua_module]
fn ljson(lua: &Lua) -> Result<Table> {
    let decode = lua.create_function(decode)?;
    let encode = lua.create_function(encode)?;
    make_exports(lua, decode, encode)
}

#[mlua::lua_module]
fn ljson_safe(lua: &Lua) -> Result<Table> {
    let decode = lua.create_function(|lua, s| match decode(lua, s) {
        Ok(v) => Ok((v, None)),
        Err(e) => Ok((Nil, Some(e.to_string()))),
    })?;
    let encode = lua.create_function(|lua, v| match encode(lua, v) {
        Ok(s) => Ok((s, None)),
        Err(e) => Ok((Nil, Some(e.to_string()))),
    })?;
    make_exports(lua, decode, encode)
}
