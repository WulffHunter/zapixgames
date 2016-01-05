use hlua::{self, Lua};

use std::fs::File;
use std::io::Read;

pub fn hello_world() {
    let mut script = String::new();

    File::open("scripts/hello.lua").unwrap()
         .read_to_string(&mut script).unwrap();

    let mut lua = Lua::new();

    lua.openlibs(); // Load standard lua libraries
    lua.set("log", hlua::function1(log)); // Add log capability

    lua.execute(&script).unwrap()
}

fn log(text: String) {
    println!("{}", text);
}
