# godot-rust

example addon integrating [Rust-lang](https://rust-lang.org) into [Godot Engine](https://godotengine.org) using [godot-rust](https://godot-rust.github.io)

this addon uses the `godot-rust` name internally

## setup

install [Rust](https://rust-lang.org/) 1.85.0+ (2024)

```sh
curl -fsSL https://sh.rustup.rs | sh
```

install [Godot Engine](https://godotengine.org/) 4.4+

## see also

[godot-rust book](https://godot-rust.github.io/book) \
[Godot Engine documentation](https://docs.godotengine.org/en/stable) \
[Learn Rust](https://www.rust-lang.org/learn) \
</x-nul>

## status

- [x] logging ([env_logger](https://github.com/rust-cli/env_logger) and [log](https://github.com/rust-lang/log))
- [x] error handling ([anyhow](https://github.com/dtolnay/anyhow))
- [x] godot singletons
- [x] godot refcounted
- [x] godot nodes
- [x] godot properties (`var` and `export`)
- [x] godot virtual functions
- [x] godot concrete functions
- [x] godot lifecycle functions (e.g. ready, process)
- [x] godot static functions (associated functions)
- [ ] signals ([unstable](https://godot-rust.github.io/book/register/signals.html))
- [ ] hot reload (needs testing)
- [ ] containerization and GitHub workflows (formatting, builds, exports, tests)

## todo

- fix [resource leaks](_archive/resource-leak.log) with `virtual_func`
- fix editor segfault and deinit not working with `--quit`
    ```sh
    cd addons/godot_rust/

    : &&
        rm -rf ../../.godot/ &&
        godot --path ../.. --editor --quit &&
    :
    ```

## debug

```sh
cd addons/godot_rust/

: &&
    cargo build &&
    godot --path ../.. --editor --quit &>/dev/null || true &&
    godot --path ../.. &&
:

code . # optional: Visual Studio Code or VSCodium
```
