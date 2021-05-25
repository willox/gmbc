# gmbc
It's a bytecode loader for gmod servers

## Usage
```lua
local bytecode = string.dump(function() print "my secrit code" end)
-- 10 minutes later
require("gmbc")
gmbc_load_bytecode(bytecode)()
```

## Building
### Dependencies
1) Rust (https://www.rust-lang.org/tools/install)
    1) Note: Rust may depend on a separate compiler on your platform, such as Visual Studio, GCC, etc:
        1) Windows: Install Visual Studio 2019
        2) Linux: Install `gcc-multilib`
2) (Optional) Additional Rust toolchains for compiling 32-bit binaries:
```sh
rustup target add i686-pc-windows-msvc # Windows
rustup target i686-unknown-linux-gnu # Linux
```

### Windows
```sh
git clone https://github.com/willox/gmbc
cd gmbc
cargo build --release --target=i686-pc-windows-msvc # 32-bit
cargo build --release --target=x86_64-pc-windows-msvc # 64-bit
```
You can then find `gmbc.dll` in `./target/<target>/release`. Rename it to `gmsv_gmbc_win32.dll` or `gmsv_gmbc_win64.dll` and you are done.

### Linux
```sh
git clone https://github.com/willox/gmbc
cd gmbc
cargo build --release --target=i686-unknown-linux-gnu # 32-bit
cargo build --release --target=x86_64-unknown-linux-gnu # 64-bit
```
You can then find `gmbc.so` in `./target/<target>/release`. Rename it to `gmsv_gmbc_linux.dll` or `gmsv_gmbc_linux64.dll` and you are done.
