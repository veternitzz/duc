use std::fs;

/// Searches the standard source code directory (``./src/``) for the specified Luau module.
/// A Luau module in this case is considered to be either a standalone file or a
/// large module where the path would be ``./src/module/init.luau``. If the module
/// was named ``foo`` the path would either be ``./src/foo.luau`` or it would be ``./src/foo/init.luau``.
pub fn module_exists(name: &'static str) -> bool {
    let basic_path = format!("./src/{}.luau", name);
    let large_path = format!("./src/{}/init.luau", name);

    if fs::exists(basic_path).unwrap() == true {
        return true
    } else if fs::exists(large_path).unwrap() == true {
        return true
    } else {
        return false
    }
}