use std::fs;

#[derive(PartialEq)]
pub enum ModuleKind {
    SIMPLE,
    COMPLEX,
    NONE
}

/// Searches the standard source code directory (``./src/``) for the specified Luau module.
/// A Luau module in this case is considered to be either a simple module or a
/// complex module where the path would be ``./src/module/init.luau``. If the module
/// was named ``foo`` the path would either be ``./src/foo.luau`` for a simple module or it would be
/// ``./src/foo/init.luau`` for a complex module. If it finds a module matching the name provided,
/// it either returns ``ModuleKind::SIMPLE``, ``ModuleKind::COMPLEX`` and if it does not find one
/// it returns ``ModuleKind::NONE``.
pub fn module_exists(name: &'static str) -> ModuleKind {
    let basic_path = format!("./src/{}.luau", name);
    let large_path = format!("./src/{}/init.luau", name);

    if fs::exists(basic_path).unwrap() == true {
        return ModuleKind::SIMPLE
    } else if fs::exists(large_path).unwrap() == true {
        return ModuleKind::COMPLEX
    } else {
        return ModuleKind::NONE
    }
}