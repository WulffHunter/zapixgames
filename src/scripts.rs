use hlua::Lua;

use std::fs::File;
use std::io::Read;

pub fn hello_world() {
    let mut script = String::new();

    File::open("scripts/hello.lua").unwrap()
         .read_to_string(&mut script).unwrap();

    let mut lua = Lua::new();
    lua.openlibs(); // Load standard lua libraries

    lua.execute(&script).unwrap()
}
