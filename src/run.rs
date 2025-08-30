use super::{runtime::Runtime, util::{self, ModuleKind}};

use mlua::prelude::*;

pub fn run(chunk: String) -> LuaResult<()> {
    let module_type = util::module_exists("run");

    if module_type == ModuleKind::COMPLEX {
        let pre_exec_duc = Runtime::new();
        pre_exec_duc.load_std()?;
        pre_exec_duc.load_file(String::from("./src/run/init.luau"))?;
    } else if module_type == ModuleKind::SIMPLE {
        let pre_exec_duc = Runtime::new();
        pre_exec_duc.load_std()?;
        pre_exec_duc.load_file(String::from("./src/run.luau"))?;
    }

    let duc = Runtime::new();
    duc.load_std()?;
    duc.load_string(chunk)?;

    Ok(())
}