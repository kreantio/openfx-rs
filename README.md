# OpenFX bindings and tools for Rust

See Also:

- [CONTRIBUTING.md](./CONTRIBUTING.md)
- [TODO.md](./TODO.md)

## Prerequisites

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

## crate `openfx`

[README.md](./crates/openfx/README.md)

Bindings for the OpenFX API in different abstraction layers.
