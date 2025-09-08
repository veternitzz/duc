use std::fs;

use mlua::prelude::*;

pub struct LuaFileMetadata {
    pub path: String,
    pub metadata: fs::Metadata
}

impl LuaFileMetadata {
    pub fn new(path: String) -> LuaResult<LuaFileMetadata> {
        Ok(LuaFileMetadata {
            path: path.clone(), 
            metadata: fs::metadata(&path)? })
    }

    pub fn size(&self) -> LuaResult<f64> {
        Ok(self.metadata.len() as f64)
    }
}

impl LuaUserData for LuaFileMetadata {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("size", |_, this, ()| -> LuaResult<f64> {
            Ok(this.size()?)
        });
    }
}