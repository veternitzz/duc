use mlua::prelude::*;

use sysinfo::{System};

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    let table = luau.create_table()?;
    table.set("hostName", luau.create_function(sys_host_name)?)?;
    table.set("name", luau.create_function(sys_name)?)?;
    table.set("memory", luau.create_function(sys_memory)?)?;
    table.set("upTime", luau.create_function(sys_uptime)?)?;
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

fn sys_memory(luau: &Lua, (): ()) -> LuaResult<LuaTable> {
    let sys = System::new_all();

    let mem_info = luau.create_table()?;
    mem_info.set("free", sys.free_memory() as f64)?;
    mem_info.set("total", sys.total_memory() as f64)?;
    mem_info.set_readonly(true);

    Ok(mem_info)
}

fn sys_uptime(_: &Lua, (): ()) -> LuaResult<f64> {
    Ok(System::uptime() as f64)
}