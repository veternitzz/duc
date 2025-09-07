use mlua::prelude::*;

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    let table = luau.create_table()?;
    table.set("bytes", luau.create_function(fmt_bytes)?)?;
    table.set("time", luau.create_function(fmt_time)?)?;
    table.set_readonly(true);

    Ok(table)
}

fn fmt_bytes(luau: &Lua, bytes: f64) -> LuaResult<LuaString> {
    if bytes > 1_000_000_000_000.0 {
        let terabytes = bytes as u64/1_000_000_000_000;
        let format_string = luau.create_string(format!("{} TB", terabytes as f64).as_bytes())?;
        
        return Ok(format_string)
    } else if bytes > 1_000_000_000.0 {
        let gigabytes = bytes as u64/1_000_000_000;
        let format_string = luau.create_string(format!("{} GB", gigabytes as f64).as_bytes())?;

        return Ok(format_string)
    } else if bytes > 1_000_000.0 {
        let megabytes = bytes as u64/1_000_000;
        let format_string = luau.create_string(format!("{} MB", megabytes as f64).as_bytes())?;

        return Ok(format_string)
    } else if bytes > 1_000.0 {
        let kilobytes = bytes as u64/1_000;
        let format_string = luau.create_string(format!("{} KB", kilobytes as f64).as_bytes())?;

        return Ok(format_string)
    }

    let format_string = luau.create_string(format!("{} B", bytes).as_bytes())?;

    Ok(format_string)
}

fn fmt_time(luau: &Lua, seconds: f64) -> LuaResult<LuaString> {
    if seconds > 86_400.0 {
        let days = seconds as u64/86_400;
        let format_string = luau.create_string(format!("{} Day(s)", days as f64).as_bytes())?;

        return Ok(format_string)
    } else if seconds > 3_600.0 {
        let hours = seconds as u64/3_600;
        let format_string = luau.create_string(format!("{} Hour(s)", hours as f64).as_bytes())?;

        return Ok(format_string)
    } else if seconds > 60.0 {
        let minutes = seconds as u64/60;
        let format_string = luau.create_string(format!("{} Minute(s)", minutes as f64).as_bytes())?;

        return Ok(format_string)
    }

    let format_string = luau.create_string(format!("{} Second(s)", seconds).as_bytes())?;
    
    Ok(format_string)
}