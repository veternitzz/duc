mod std_globals;
mod std_io;
mod std_fs;

use mlua::prelude::*;
use std::{fs, env};

fn main() -> LuaResult<()> {
    // Do command line arguments
    let args: Vec<String> = env::args().collect();

    // Read the file the user passes as an arg
    let source = fs::read(&args[1])?;

    // Create a new Lua state
    let luau = Lua::new();

    // Inject globals
    std_globals::inject(&luau)?;

    // Inject std libs for testing
    std_io::inject(&luau)?;
    std_fs::inject(&luau)?;

    // Load and execute the Luau file
    luau.load(source)
        .exec()?;

    Ok(())
}