# Examples for `openfx::{low, low_plugin}`

See [../README.md](../README.md) for the index.

## Extract an example crate

If you want to extract an example crate under this folder, perhaps to use it as
a base for developing your own plugin, you can:

1. Copy the folder of the crate you want to extract, together with `shared/`, to
   your location.
2. Make sure that `PATH_BUNDLE_SH` in the extracted crate's `justfile` points to
   the correct location of `shared/scripts/bundle.sh`.
3.
   - For `official-guide-example-plugins`, you can delete `shared/support`.
   - For other example crates, you should
     - either use a cargo workspace and point the path of
       `workspace.dependencies.openfx-layer-low-examples-support` to
       `shared/support`, for example:
       ```toml
       openfx-layer-low-examples-support = { path = "<path-to-shared>/support", version = "<version>" }
       ```
     - or integrate the code from `shared/support` directly into your crate
       yourself.
