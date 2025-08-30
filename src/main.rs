mod std_lib;
mod cli;
mod runtime;

use mlua::prelude::*;

fn main() -> LuaResult<()> {
    // Create a new duc runtime state
    let duc = runtime::Runtime::new();
    duc.load_std()?;

    // Do command line arguments
    let arg_return = cli::process()?;

    if arg_return == "none" {
        return Ok(())
    }

    // Execute the loaded file contents
    // if the user chose to run a file
    duc.load_string(arg_return)?;

    Ok(())
}