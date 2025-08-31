mod std_lib;
mod cli;
mod runtime;
mod util;
mod luau_modules;

use luau_modules::run;

use mlua::prelude::*;

fn main() -> LuaResult<()> {
    // Do command line arguments
    let arg_return = cli::process()?;

    if arg_return == "none" {
        return Ok(())
    }

    // Use the ``run`` utility module to execute the project
    run::run(arg_return)?;

    Ok(())
}