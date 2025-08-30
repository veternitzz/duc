#![allow(dead_code)]

use super::std_lib;

use std::fs;

use mlua::prelude::*;

pub struct Runtime {
    pub luau_state: Lua
}

impl Runtime {
    pub fn new() -> Runtime {
        let luau = Lua::new_with(LuaStdLib::ALL, LuaOptions::new()).unwrap();

        Runtime {
            luau_state: luau
        }
    }

    pub fn load_std(&self) -> LuaResult<()> {
        std_lib::inject(&self.luau_state)?;

        Ok(())
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