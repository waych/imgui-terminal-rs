# imgui-terminal-rs: bindings for imgui-terminal

This crate provides simple bindings for
[imgui-terminal](https://github.com/timmoorhouse/imgui-terminal), a terminal
implementation for [Dear ImGui](https://github.com/ocornut/imgui).  It is meant
to be used in conjunction with the
[imgui-rs](https://github.com/imgui-rs/imgui-rs) crate.

As of the `0.1.0` release, this crate doesn't do much more than provide simple
bindings to the C++ code enabling the terminal to be drawn and basic IO to
function.  Further work is still required to address various TODOs both on the
rust side as well as on the C++ to make the terminal usable.

## How to contribute

1. Change or add something
2. Make sure you're using the latest stable Rust
3. Run rustfmt to guarantee code style conformance

    ```bash
    rustup component add rustfmt
    cargo fmt
    ```

4. Open a pull request in Github

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Uses [Dear ImGui](https://github.com/ocornut/imgui) and
[imgui-terminal](https://github.com/timmoorhouse/imgui-terminal).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
