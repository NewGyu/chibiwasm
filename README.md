# What's this?

This is just only for practice to learn WASM specification and Rust lang.

Thanks:
- https://qiita.com/kgtkr/items/f4b3e2d83c7067f3cfcb
    - https://github.com/kgtkr/wasm-rs/tree/adc-2019-12-22
- https://github.com/skanehira/chibiwasm


# Structure

## Crates
There are two crates in this repository.
- binary crate ... depends on library crate
- library crate  ... almost all codes are contained.

## modules

### crate::wasm::structure
Implementations that according to [the structure definition](https://webassembly.github.io/spec/core/syntax/index.html) such as Values, Types, Instructions and Modules.


### crate::wasm::binary

Extensions for the structure to decode from wasm binary according to [Binary Format section](https://webassembly.github.io/spec/core/binary/index.html). In many cases, those are implemented as [TryFrom](https://doc.rust-lang.org/stable/std/convert/trait.TryFrom.html).