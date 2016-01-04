extern crate hlua;

use hlua::Lua;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut script = String::new();

    File::open("scripts/hello.lua").unwrap()
         .read_to_string(&mut script).unwrap();

    let mut lua = Lua::new();
    lua.openlibs(); // Load standard lua libraries

    lua.execute(&script).unwrap()
}
