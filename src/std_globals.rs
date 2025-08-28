use mlua::prelude::*;
use std::{io::{self, Write}};

pub fn inject(luau: &Lua) -> LuaResult<()> {
    let globals = luau.globals();
    globals.set("warn", luau.create_function(warn)?)?;
    globals.set("print", luau.create_function(print)?)?;

    Ok(())
}

fn warn(_luau: &Lua, s: String) -> LuaResult<()> {
    println!("warn: {}", s);
    Ok(())
}

fn print(_: &Lua, s: String) -> LuaResult<()> {
    println!("{}", s);
    Ok(())
}
