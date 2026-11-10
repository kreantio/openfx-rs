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

### Plugin Making: `official-example-plugins`

These are the ports of the example plugins under the official repository's
[`Examples` folder].

| Plugin Name                                    | Code (`layer-low`) |
| ---------------------------------------------- | ------------------ |
| Basic (`OFX Gain Example`)                     | TODO               |
| ChoiceParams (`OFX Choice Param Example`)      | TODO               |
| ColourSpace (…)                                | TODO               |
| Custom (`OFX Custom Param Example`)            | TODO               |
| DepthConverter (`OFX Depth Converter Example`) | TODO               |
| DrawSuite (`OFX DrawSuite Example`)            | TODO               |
| Invert (`OFX Invert Example`)                  | TODO               |
| OpenGL (`OFX OpenGL Example`)                  | TODO               |
| Overlay (`OFX Overlay Example`)                | TODO               |
| Rectangle (`OFX Rectangle Example`)            | TODO               |
| Test (`OFX Test Properties`)                   | TODO               |

### Plugin Making: `official-old-support-example-plugins`

These are the ports of the example plugins under the official repository's
[`Support/Plugins` folder].

| Plugin Name                               | Code (`layer-low`) |
| ----------------------------------------- | ------------------ |
| Basic (`Gain`)                            | TODO               |
| ChoiceParams (`Choice Params (Support)`)  | TODO               |
| Field (`Field`)                           | TODO               |
| GPUGain (`GPU Gain`)                      | TODO               |
| Generator (`Noise`)                       | TODO               |
| Invert (`Invert`)                         | TODO               |
| MultiBundle (`Gamma` and `Dot Generator`) | TODO               |
| Retimer (`Retimer`)                       | TODO               |
| Tester (`GenericTest`)                    | TODO               |
| Transition (`Cross Fade`)                 | TODO               |
| ../PropTester (`Prop Tester`)             | TODO               |

[OpenFX Programming Guide]: https://openfx.readthedocs.io/en/main/Guide/index.html
[`Examples` folder]: https://github.com/AcademySoftwareFoundation/openfx/tree/main/Examples
[`Support/Plugins` folder]: https://github.com/AcademySoftwareFoundation/openfx/tree/main/Support/Plugins
