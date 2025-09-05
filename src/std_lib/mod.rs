pub mod std_fs;
pub mod std_io;
pub mod std_globals;

use mlua::prelude::*;

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    let table = luau.create_table()?;
    table.set("fs", std_fs::create(&luau)?)?;
    table.set("io", std_io::create(&luau)?)?;

    Ok(table)
}