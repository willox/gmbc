use std::{ffi::CString, os::raw::c_char};

mod api;

#[repr(C)]
#[derive(Clone)]
pub struct State(*const std::ffi::c_void);

pub type Function = unsafe extern "C" fn (state: State) -> i32;

pub const LUA_GLOBALSINDEX: i32 = -10002;

impl State {
    pub unsafe fn pop(&self) {
        self.pop_n(1);
    }

    pub unsafe fn pop_n(&self, count: i32) {
        self.set_top(-count - 1);
    }

    pub unsafe fn set_top(&self, index: i32) {
        (api::INSTANCE.lua_settop)(self.to_owned(), index)
    }

    pub unsafe fn push_value(&self, index: i32) {
        (api::INSTANCE.lua_pushvalue)(self.to_owned(), index)
    }

    pub unsafe fn replace(&self, index: i32) {
        (api::INSTANCE.lua_replace)(self.to_owned(), index)
    }

    pub unsafe fn push_globals(&self) {
        (api::INSTANCE.lua_pushvalue)(self.to_owned(), LUA_GLOBALSINDEX)
    }

    pub unsafe fn push_string(&self, data: &str) {
        (api::INSTANCE.lua_pushlstring)(self.to_owned(), data.as_ptr() as *const c_char, data.len())
    }

    pub unsafe fn push_string_binary(&self, data: &[u8]) {
        (api::INSTANCE.lua_pushlstring)(self.to_owned(), data.as_ptr() as *const c_char, data.len())
    }

    pub unsafe fn push_function(&self, func: Function) {
        (api::INSTANCE.lua_pushcclosure)(self.to_owned(), func, 0)
    }

    pub unsafe fn set_table(&self, index: i32) {
        (api::INSTANCE.lua_settable)(self.to_owned(), index)
    }

    pub unsafe fn get_string_data(&self, index: i32) -> Option<&[u8]> {
        let mut len: usize = 0;
        let ptr = (api::INSTANCE.lua_tolstring)(self.to_owned(), index, &mut len);

        if ptr.is_null() {
            return None;
        }

        Some(std::slice::from_raw_parts(ptr as *const u8, len))
    }

    pub unsafe fn load(&self, code: &[u8], name: &str) -> i32 {
        let name = CString::new(name).unwrap();
        (api::INSTANCE.lual_loadbuffer)(self.to_owned(), code.as_ptr() as *const i8, code.len(), name.as_ptr())
    }
}
