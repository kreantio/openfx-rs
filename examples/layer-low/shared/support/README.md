# `openfx-layer-low-examples-support`

> [!NOTE]
> `official-guide-example-plugins` intentionally does not use this crate, so
> that it stays self-contained and learners can stay focused on the code in the
> guide itself.

This crate holds code shared across the example crates for
`openfx::{low, low_plugin}` to reduce code duplication.

This crate is not intended to be used outside of the example crates, and it
guarantees neither complete functionality coverage nor a smooth development
experience. However, if you really want to use it, please vendor it.

This crate is published at <https://crates.io> only to prevent people who base
their work on one of the example crates from accidentally fetching this crate
from crates.io instead of vendoring it.
