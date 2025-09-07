use mlua::prelude::*;

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    let table = luau.create_table()?;
    table.set_readonly(true);

    Ok(table)
}