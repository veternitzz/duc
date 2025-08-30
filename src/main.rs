mod std_lib;
mod cli;

use mlua::prelude::*;

fn main() -> LuaResult<()> {
    // Do command line arguments
    let arg_return = cli::process()?;

    if arg_return == "none" {
        return Ok(())
    }

    // Create a new Luau state
    let luau = Lua::new();

    // Inject the duc standard library into the Luau state
    std_lib::inject(&luau)?;

    // Load and execute the Luau file
    luau.load(arg_return.as_bytes())
        .exec()?;

    Ok(())
}