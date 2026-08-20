# OpenFX bindings and tools for Rust

## Prerequisites

### Building plugins with the crate `openfx`

The crate `openfx` itself is dependency-free, but OpenFX plugins require to be
packed in a specific way. Therefore, you need to do some additional work after
the binary is built. See: [OpenFX reference / Packaging OFX Plug-ins]. You can
also have a look at
[some of the scripts I use for bundling the example plugins].

[OpenFX reference / Packaging OFX Plug-ins]: https://openfx.readthedocs.io/en/latest/Reference/ofxPackaging.html
[some of the scripts I use for bundling the example plugins]: examples/ofx-guide-example-plugins-layer-sys/scripts/bundle.ts

Please don't forget that the `Cargo.toml` of your plugin crates should contain:

```toml
[lib]
crate-type = ["cdylib"]
```

### Development

The following tools are required:

| tool(s)                           | building examples? | updating generated code in crate `openfx`? |
| --------------------------------- | ------------------ | ------------------------------------------ |
| POSIX tools (`rm`, `mkdir`, etc.) | yes (I assume)     | yes                                        |
| [`just`]                          | yes                | yes                                        |
| [`deno`]                          | yes                | yes                                        |
| [`clang++`]                       | no                 | yes                                        |

[`just`]: https://github.com/casey/just
[`deno`]: https://deno.com/
[`clang++`]: https://clang.llvm.org/

## crate [`openfx`](./crates/openfx/)

Bindings for the OpenFX API in different abstraction layers.
