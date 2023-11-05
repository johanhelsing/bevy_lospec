# bevy_lospec

Asset loader plugin for Bevy that adds support for lospec color palette formats.

## Usage

Find a palette on <https://lospec.com>, note down the name.

Download its json from: `https://lospec.com/palette-list/<palette-name>.json`

Add the plugin:

```rust
app.add_plugins(PalettePlugin);
```

```rust
let palette: Handle<Palette> = asset_server.load_asset("palette.json");
```

See the [`simple`](./examples/simple.rs) example for details

## Bevy version support

The `main` branch targets the latest bevy release.

|bevy|bevy_lospec|
|----|-----------|
|0.12|0.6, main  |
|0.11|0.5        |
|0.10|0.4        |
|0.9 |0.3        |
|0.8 |0.2        |
|0.7 |0.1        |

## License

All code in this repository dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
