# OpenFX bindings and tools for Rust

> [!NOTE]
> Recently (2026/09/26), this project's git history was rewritten to remove some
> files. See [this issue](https://github.com/kreantio/openfx-rs/issues/6) for
> details. At the time, the project had 0 forks and 0 merged PRs, so no one
> should be affected. Refs for branches and tags have been manually remapped.
> Thanks for the understanding.

See Also:

- [ATTRIBUTION.md](./ATTRIBUTION.md)
- [CONTRIBUTING.md](./CONTRIBUTING.md)

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

[![Latest version](https://img.shields.io/crates/v/openfx.svg)](https://crates.io/crates/openfx)
[![Documentation](https://docs.rs/openfx/badge.svg)](https://docs.rs/openfx)
[![Crates.io License](https://img.shields.io/crates/l/openfx)](https://github.com/kreantio/openfx-rs/blob/main/LICENSE)

[README.md](./crates/openfx/README.md)

Bindings for the OpenFX API in different abstraction layers.

## Internal Crates

| Name                       | Published? | Description                                           |
| -------------------------- | ---------- | ----------------------------------------------------- |
| [`openfx-internal-macros`] | Yes        | internal procedural macros for the crate `openfx`     |
| [`openfx-codegen`]         | No         | a CLI tool for generating code for the crate `openfx` |

[`openfx-internal-macros`]: crates/openfx-internal-macros
[`openfx-codegen`]: crates/private/openfx-codegen
