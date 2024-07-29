# `cortex-m-quickstart-nucleo-stm32g474`

This is a fork of https://github.com/rust-embedded/cortex-m-quickstart where I
get some of their examples running on a [NUCLEO-STM32G474RE
board.](https://www.st.com/en/evaluation-tools/nucleo-g474re.html)

The goal of this repo is to make a few toy low level embedded stable rust programs that run on hardware. **There is support for different STM32 nucleo boards on different branches.**

I'm sticking to low-level hardware abstractions by only using a `rust2svd`-based
crate (`stm32g4` in this case) for hardware abstraction.
I wrote these while following along with the [rust-embedded
book](https://docs.rust-embedded.org/book/start/hardware.html)

## Dependencies

I used the following to build & run these embedded programs on Ubuntu 24.

- Rust 1.78, stable toolchain.
- `gdb-multiarch` & `openocd` to run the programs on target hardware.
- Install udev rules per the [embedded book instructions](https://docs.rust-embedded.org/book/intro/install/linux.html#udev-rules) if this is the first time you've used
this devkit hardware.
- `rust-std` components (pre-compiled `core` crate) for ARM Cortex-M4F
  target. Run:

``` console
$ rustup target add thumbv7em-none-eabihf
```

Check out the book if that doesn't work `https://docs.rust-embedded.org/book/intro/install.html`

## Running these Examples

1. Build the examples in release and debug profiles with the following commands.
   The crash example only works in the 'release' profile, so that's why I build
   both profiles.
``` bash
cargo build --examples
cargo build --release --examples
```
2. Connect the USB ST-LINK port of the NUCLEO-STM32G474RE board to a USB port
  on your computer.
3. From a terminal in the root of this repo, run `openocd`. That will pick up
  the configuration from openocd.cfg for connecting to your devkit.
4. From a separate terminal, run a gdb command that looks like one of these.
 Make sure to pick up gdb configuration with `-x openocd.gdb`.
``` console
$ gdb-multiarch -x openocd.gdb <path_to_example_binary>

$ gdb-multiarch -x openocd.gdb target/thumbv7em-none-eabihf/debug/examples/mtime
# release example build
$ gdb-multiarch -x openocd.gdb target/thumbv7em-none-eabihf/release/examples/crash
```

# License

These examples is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option(same as the template).
