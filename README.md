# lurien

[![Crates.io](https://img.shields.io/crates/v/lurien.svg)](https://crates.io/crates/lurien)
[![Docs.rs](https://docs.rs/lurien/badge.svg)](https://docs.rs/lurien)
[![CI](https://github.com/tsoutsman/lurien/workflows/CI/badge.svg)](https://github.com/tsoutsman/lurien/actions)

## `lurien` is a work in progress; it does not yet work.

`lurien` is a tool to help manage config files on multiple devices. It allows you to insert
markers, similar to a templating language, that define what content should be enabled for
what device.

**When editing files that contain markers, you must either run `lurien populate` or `lurien watch`.**
This is because the `.lurien` file tracks the line number of the markers, and so if you change the
file without markers, when run `lurien apply` the snippets placed in will be offset. `lurien watch`
just watches for any changes in the directory and continually updates the file. If you are
working with the markers directly, it is suggested you run `lurien populate` so that you can see
the markers. If you are changing a part of the file that does not have markers, you can just
run `lurien watch`.

## Example

Given a file as follows:

```yaml
font_family     Fira Code Nerd Font
{{#lurien desktop}}
font_size       21.0
{{/lurien desktop}}
{{#lurien laptop}}
font_size       14.0
{{/lurien laptop}}
```

You can run `lurien save`, which will save the snippets to a `.lurien` file. The
file will now contain the following:

```yaml
font_family     Fira Code Nerd Font
```

To apply the `desktop` configuration, run `lurien apply desktop`
which will result in:

```yaml
font_family     Fira Code Nerd Font
font_size       21.0
```

To add all the markers back to the file, run `lurien populate`, which will result in:

```yaml
font_family     Fira Code Nerd Font
{{#lurien desktop}}
font_size       21.0
{{/lurien desktop}}
{{#lurien laptop}}
font_size       14.0
{{/lurien laptop}}
```

the same file contents that you started with.

## Installation

### Cargo

- Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
- run `cargo install lurien`

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
