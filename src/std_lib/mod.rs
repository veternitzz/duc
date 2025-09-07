pub mod std_fs;
pub mod std_io;
pub mod std_globals;
pub mod std_sys;
pub mod std_fmt;

use mlua::prelude::*;

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    let table = luau.create_table()?;
    table.set("fs", std_fs::create(&luau)?)?;
    table.set("io", std_io::create(&luau)?)?;
    table.set("sys", std_sys::create(&luau)?)?;

    Ok(table)
}