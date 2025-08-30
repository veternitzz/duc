mod std_fs;
mod std_globals;
mod std_io;

use mlua::prelude::*;

pub fn inject(luau: &Lua) -> LuaResult<()> {
    std_fs::inject(luau)?;
    std_globals::inject(luau)?;
    std_io::inject(luau)?;

    Ok(())
}