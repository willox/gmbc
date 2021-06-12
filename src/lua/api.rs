use std::os::raw::c_char;

use libloading::{Library, Symbol};
use super::{State, Function};

lazy_static::lazy_static! {
    pub static ref INSTANCE: Functions = {
        Functions::new()
    };
}

type Int = std::os::raw::c_int;
type Size = usize;

pub struct Functions {
    pub lua_settop: Symbol<'static, unsafe extern "C" fn(state: State, count: Int)>,
    pub lua_pushvalue: Symbol<'static, unsafe extern "C" fn(state: State, index: Int)>,
    pub lua_replace: Symbol<'static, unsafe extern "C" fn(state: State, index: Int)>,
    pub lua_pushlstring: Symbol<'static, unsafe extern "C" fn(state: State, data: *const c_char, length: Size)>,
    pub lua_pushcclosure: Symbol<'static, unsafe extern "C" fn(state: State, func: Function, upvalues: Int)>,
    pub lua_settable: Symbol<'static, unsafe extern "C" fn(state: State, index: Int)>,
    pub lua_tolstring: Symbol<'static, unsafe extern "C" fn(state: State, index: Int, out_size: *mut Size) -> *const c_char>,
    pub lual_loadbuffer: Symbol<'static, unsafe extern "C" fn(state: State, code: *const c_char, length: Size, name: *const c_char) -> i32>,
}

impl Functions {
    fn new() -> Self {
        unsafe {
            let library = Box::new(Self::find_library());
            let library = Box::leak(library); // Keep this library referenced forever

            Functions {
                lua_settop: Self::find_symbol(library, b"lua_settop"),
                lua_pushvalue: Self::find_symbol(library, b"lua_pushvalue"),
                lua_replace: Self::find_symbol(library, b"lua_replace"),
                lua_pushlstring: Self::find_symbol(library, b"lua_pushlstring"),
                lua_pushcclosure: Self::find_symbol(library, b"lua_pushcclosure"),
                lua_settable: Self::find_symbol(library, b"lua_settable"),
                lua_tolstring: Self::find_symbol(library, b"lua_tolstring"),
                lual_loadbuffer: Self::find_symbol(library, b"luaL_loadbuffer"),
            }
        }
    }

    unsafe fn find_symbol<T>(library: &'static Library, symbol: &[u8]) -> Symbol<'static, T> {
        library.get(symbol).unwrap()
    }

    #[cfg(target_os = "windows")]
    unsafe fn find_library() -> Library {
        Library::new("lua_shared.dll").unwrap()
    }

    #[cfg(target_os = "linux")]
    unsafe fn find_library() -> Library {
        Library::new("lua_shared_srv.so")
        .or_else(|_| Library::new("lua_shared.so"))
        .or_else(|_| Library::new("garrysmod/bin/lua_shared_srv.so"))
        .or_else(|_| Library::new("garrysmod/bin/lua_shared.so"))
        .or_else(|_| Library::new("bin/lua_shared_srv.so"))
        .or_else(|_| Library::new("bin/lua_shared.so"))
        .or_else(|_| Library::new("../lua_shared_srv.so"))
        .or_else(|_| Library::new("../lua_shared.so"))
        .unwrap()
    }
}
