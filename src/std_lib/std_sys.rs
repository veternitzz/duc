use mlua::prelude::*;

use sysinfo::{System};

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    let table = luau.create_table()?;
    table.set("hostname", luau.create_function(sys_host_name)?)?;
    table.set("name", luau.create_function(sys_name)?)?;
    table.set_readonly(true);

    Ok(table)
}

fn sys_host_name(luau: &Lua, (): ()) -> LuaResult<LuaString> {
    let host_name = System::host_name().unwrap();

    Ok(luau.create_string(host_name.as_bytes())?)
}

fn sys_name(luau: &Lua, (): ()) -> LuaResult<LuaString> {
    let name = System::name().unwrap();

    Ok(luau.create_string(name.as_bytes())?)
}