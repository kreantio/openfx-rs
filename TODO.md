# TODO

## Crate `openfx`

- [x] `sys`
- [x] `sys_helpers`
- [ ] `low`
  - [ ] `low_plugin`: WIP
  - [ ] `low_host`
  - [ ] handle map-like properties (`OfxImageEffectPropFrameRange_*` and
        `OfxImageEffectClipPropRoI_*`).
  - [ ] stronger typing. (e.g., `Time` (`OfxTime`) instead of `f64`, `RectI`
        (`OfxRectI`) instead of `[c_int; 4]`, etc.)
  - [ ] improve DX of Go to Definition: move `generated/` to `src/generated/`,
        generate `mod.rs` files, and replace `include!` with `pub use`?
  - [ ] make grammars of macros that do similar things more consistent.
    - [ ] between `low_make_property_enums` and `low_make_property_enums_from_c`
- [ ] `high`

## Examples

- [`ofx-guide-example-plugins`]:
  - [x] layer `sys`
  - [ ] layer `low`: WIP
  - [ ] layer `high`
- [ ] ports of [official examples] (not guide examples)
  - using `glow` for GL-related examples
- [ ] a port of [`learn-wgpu`] (Metal on macOS in supported hosts?)
- [ ] a simple host that supports OpenGL rendering (using `glow`?)
- [ ] [`ntsc-rs`] + layer `high`

[`ofx-guide-example-plugins`]: https://openfx.readthedocs.io/en/latest/Guide/index.html
[official examples]: ./crates/openfx/vendor/openfx/Examples/
[`learn-wgpu`]: https://github.com/umajho/ludejo-de-umajho/tree/1e933c930f0d140b61e8827b89c4712c3ebe562d/ui/deno-wasm-wgpu/crates/learn_wgpu_tutorial
[`ntsc-rs`]: https://github.com/ntsc-rs/ntsc-rs/tree/af9833b4bb81f195f7fe4a3667211f2a94139a42/crates/openfx-plugin
