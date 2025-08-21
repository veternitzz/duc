use std::io::{self, Write};
use mlua::prelude::*;

pub fn inject(luau: &Lua) -> LuaResult<()> {
    let globals = luau.globals();

    let table = luau.create_table().unwrap();
    table.set("write", luau.create_function(io_write)?)?;
    table.set("ewrite", luau.create_function(io_ewrite)?)?;
    table.set("readline", luau.create_function(io_readline)?)?;
    table.set_readonly(true);

    globals.set("io", table)?;
    Ok(())
}

fn io_write(_: &Lua, s: String) -> LuaResult<()> {
    let mut stdout = io::stdout();
    let bytes = s.as_bytes();

    stdout.write(bytes)?;
    Ok(())
}

fn io_ewrite(_: &Lua, s: String) -> LuaResult<()> {
    let mut stderr = io::stderr();
    let bytes = s.as_bytes();

    stderr.write(bytes)?;
    Ok(())
}

fn io_readline(luau: &Lua, (): ()) -> LuaResult<LuaString> {
    let stdin = io::stdin();
    let mut out = String::new();

    stdin.read_line(&mut out)?;
    luau.create_string(&out)
}
