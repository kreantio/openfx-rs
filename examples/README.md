# Example Index

This folder contains examples for the projects in this repository. Every example
is located two folders below this directory.

The name of an outer folder indicates which project the example is for:

- [`layer-low`](./layer-low/): The examples demonstrate how to write OpenFX
  plugins using `openfx::{low, low_plugin}`.

The name of an inner folder indicates the topic of the example.

## Examples Categorized by Topic

### Plugin Making: `official-guide-example-plugins`

These are the ports of the example plugins in the official
[OpenFX Programming Guide].

| Plugin Name              | Code (`layer-low`)                                                                                |
| ------------------------ | ------------------------------------------------------------------------------------------------- |
| `OFX Basics Example`     | [`plugin_1_basics.rs`](./layer-low/official-guide-example-plugins/src/plugin_1_basics.rs)         |
| `OFX Invert Example`     | [`plugin_2_invert.rs`](./layer-low/official-guide-example-plugins/src/plugin_2_invert.rs)         |
| `OFX Gain Example`       | [`plugin_3_gain.rs`](./layer-low/official-guide-example-plugins/src/plugin_3_gain.rs)             |
| `OFX Saturation Example` | [`plugin_4_saturation.rs`](./layer-low/official-guide-example-plugins/src/plugin_4_saturation.rs) |
| `OFX Circle Example`     | [`plugin_5_circle.rs`](./layer-low/official-guide-example-plugins/src/plugin_5_circle.rs)         |

[OpenFX Programming Guide]: https://openfx.readthedocs.io/en/main/Guide/index.html
