# OpenFX bindings and tools for Rust

See Also:

- [CONTRIBUTING.md](./CONTRIBUTING.md)
- [TODO.md](./TODO.md)

## Prerequisites

The following tools are required:

| Tool(s)                           | Building Examples? | Updating Generated Code in Crate `openfx`? |
| --------------------------------- | ------------------ | ------------------------------------------ |
| POSIX tools (`rm`, `mkdir`, etc.) | yes (I assume)     | yes                                        |
| [`just`]                          | yes                | yes                                        |
| [`deno`]                          | yes                | yes                                        |
| [`clang++`]                       | no                 | yes                                        |

[`just`]: https://github.com/casey/just
[`deno`]: https://deno.com/
[`clang++`]: https://clang.llvm.org/

## Crate `openfx`

[<img alt="github" src="https://img.shields.io/badge/github-kreantio/openfx-rs?logo=github" height="20">](https://github.com/kreantio/openfx-rs)
[![Latest version](https://img.shields.io/crates/v/openfx.svg)](https://crates.io/crates/openfx)
[![Documentation](https://docs.rs/openfx/badge.svg)](https://docs.rs/openfx)
[![Crates.io License](https://img.shields.io/crates/l/openfx)](https://github.com/kreantio/openfx-rs/blob/main/LICENSE.md)

[README.md](./crates/openfx/README.md)

Bindings for the OpenFX API in different abstraction layers.

## Internal Crates

| Name                       | Published? | Description                                           |
| -------------------------- | ---------- | ----------------------------------------------------- |
| [`openfx-internal-macros`] | Yes        | internal procedural macros for the crate `openfx`     |
| [`openfx-codegen`]         | No         | a CLI tool for generating code for the crate `openfx` |

[`openfx-internal-macros`]: crates/openfx-internal-macros
[`openfx-codegen`]: crates/private/openfx-codegen
