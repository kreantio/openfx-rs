# Crate `opk`

> [!NOTE]
> Not ready yet.

`opk` (OpenFX Plugin Kit) is an opinionated yet flexible framework for
developing OpenFX plugins built on top of [the `openfx` crate].

### Design Principles

- **Opinionated**:
  - In most cases, developing plugins with this crate should be straightforward.
  - The components provided by this crate (e.g., for dealing with contexts,
    parameters, etc.) should cover common plugin development needs and have
    sensible defaults.
- **Flexible**:
  - Escape hatches to [the `openfx` crate] should always be available for
    uncovered or advanced use cases. Nothing you can do with the `openfx` crate
    should be blocked by this crate.
  - Using the components provided by this crate should not be mandatory.
  - The components provided by this crate should be extensible.
- **Explicit**:
  - Known actions from the host should map one-to-one to methods on traits like
    `Plugin`.
  - Except for trait default implementations, host function invocations should
    always originate from the code that uses this crate, rather than implicitly
    from within the crate (e.g., action handling) itself.

### Examples

See also: [$REPO_ROOT/examples/README.md](../../examples/README.md)

[the `openfx` crate]: https://crates.io/crates/openfx
