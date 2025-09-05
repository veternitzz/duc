#![allow(dead_code)]

use crate::std_lib::std_globals;

use std::fs;

use mlua::prelude::*;

pub struct Runtime {
    pub luau_state: Lua
}

impl Runtime {
    pub fn new() -> Runtime {
        let luau = Lua::new_with(LuaStdLib::ALL, LuaOptions::new()).unwrap();
        std_globals::inject(&luau).unwrap();

        Runtime {
            luau_state: luau
        }
    }

    pub fn load_string(&self, chunk: String) -> LuaResult<()> {
        let luau = &self.luau_state;

        luau.load(chunk)
            .exec()?;

        Ok(())
    }

    pub fn load_file(&self, path: String) -> LuaResult<()> {
        let contents = fs::read_to_string(path)?;
        let luau = &self.luau_state;

        luau.load(contents.as_bytes())
            .exec()?;
        
        Ok(())
    }

}