use std::os::unix::ffi::OsStrExt;

use mlua::prelude::*;

use gethostname::gethostname;

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    let table = luau.create_table()?;
    table.set("hostname", luau.create_function(sys_host_name)?)?;
    table.set_readonly(true);

    Ok(table)
}

fn sys_host_name(luau: &Lua, (): ()) -> LuaResult<LuaString> {
    let host_name = gethostname();
    
    Ok(luau.create_string(host_name.as_bytes())?)
}