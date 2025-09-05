use std::io::{self, Write};
use mlua::prelude::*;

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    let table = luau.create_table().unwrap();
    table.set("write", luau.create_function(io_write)?)?;
    table.set("ewrite", luau.create_function(io_ewrite)?)?;
    table.set("readLine", luau.create_function(io_readline)?)?;
    table.set_readonly(true);

    Ok(table)
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
