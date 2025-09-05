mod std_lib;
mod cli;
mod runtime;
mod util;
mod luau_modules;

use mlua::prelude::*;

fn main() -> LuaResult<()> {
    // Do command line arguments
    let arg_return = cli::process()?;

    if arg_return == "none" {
        return Ok(())
    }

    let duc = runtime::Runtime::new();
    duc.open_globals()?;
    duc.load_string(arg_return)?;

    Ok(())
}