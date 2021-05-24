mod lua;

unsafe extern "C" fn load_bytecode(state: lua::State) -> i32 {
    match state.get_string_data(1) {
        Some(code) => {
            if state.load(code, "loaded bytecode") != 0 {
                match state.get_string_data(-1) {
                    None => state.push_string("fucked up"),
                    Some(ptr) => {
                        state.push_string_binary(ptr);
                    }
                }
                return 1;


            }

            1
        }

        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn gmod13_open(state: lua::State) -> i32 {
    state.push_globals();
    state.push_string("gmbc_load_bytecode");
    state.push_function(load_bytecode);
    state.set_table(-3);
    state.pop();
    0
}

#[no_mangle]
pub extern "C" fn gmod13_close(_state: lua::State) -> i32 {
    0
}
