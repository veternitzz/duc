use mlua::prelude::*;

use super::{std_fs, std_globals, std_io};

pub fn inject(luau: &Lua) -> LuaResult<()> {
    std_fs::inject(luau)?;
    std_globals::inject(luau)?;
    std_io::inject(luau)?;

    Ok(())
}