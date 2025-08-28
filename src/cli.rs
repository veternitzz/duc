use std::{fs, env};

use mlua::prelude::*;

pub fn process() -> LuaResult<String> {
    let args: Vec<String> = env::args().collect();

    if args[1] == "run" {
        return Ok(run(&args)?)
    } else if args[1] == "init" {
        return Ok(init()?)
    } else {
        return Err(mlua::Error::runtime("invalid argument at position 1"))
    }
}

fn run(args: &Vec<String>) -> LuaResult<String> {
    if args.len() < 3 {
        let contents = fs::read_to_string("./srcd/init.luau")?;

        return Ok(contents)
    }
    
    if !fs::exists(&args[2])? {
        return Err(mlua::Error::runtime("attempted to run a file that does not exist"))
    }

    let contents = fs::read_to_string(&args[2])?;

    Ok(contents)
}

fn init() -> LuaResult<String> {
    let source = "print('Hello, World!')";

    fs::create_dir("./srcd")?;
    fs::write("./srcd/init.luau", source)?;

    Ok(String::from("none"))
}