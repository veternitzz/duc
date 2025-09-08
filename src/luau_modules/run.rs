use std::fs;

use crate::{runtime::Runtime, util::{self, ModuleKind}};

use mlua::prelude::*;

pub fn run() -> LuaResult<()> {
    let module_type = util::module_exists("run");

    let pre_exec_duc = Runtime::new();
    pre_exec_duc.open_globals()?;
    pre_exec_duc.open_types()?;

    if module_type == ModuleKind::COMPLEX && fs::exists("./src/run/init.luau")? {
        pre_exec_duc.load_file(String::from("./src/run/init.luau"))?;
    } else if module_type == ModuleKind::SIMPLE && fs::exists("./src/run.luau")? {
        pre_exec_duc.load_file(String::from("./src/run.luau"))?;
    }

    Ok(())
}