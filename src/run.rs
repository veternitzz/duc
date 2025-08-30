use super::{runtime::Runtime, util::{self, ModuleKind}};

use mlua::prelude::*;

pub fn run(chunk: String) -> LuaResult<()> {
    let module_type = util::module_exists("run");

    let pre_exec_duc = Runtime::new();
    pre_exec_duc.load_std()?;

    if module_type == ModuleKind::COMPLEX {
        pre_exec_duc.load_file(String::from("./src/run/init.luau"))?;
    } else if module_type == ModuleKind::SIMPLE {;
        pre_exec_duc.load_file(String::from("./src/run.luau"))?;
    }

    let duc = Runtime::new();
    duc.load_std()?;
    duc.load_string(chunk)?;

    Ok(())
}