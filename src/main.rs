mod std_globals;
mod std_io;
mod std_fs;
mod std_lib;

use mlua::prelude::*;
use std::{fs, env};

fn main() -> LuaResult<()> {
    // Do command line arguments
    let args: Vec<String> = env::args().collect();

    // Read the file the user passes as an arg
    let source = fs::read(&args[1])?;

    // Create a new Luau state
    let luau = Lua::new();

    // Inject the duc standard library into the Luau state
    std_lib::inject(&luau)?;

    // Load and execute the Luau file
    luau.load(source)
        .exec()?;

    Ok(())
}