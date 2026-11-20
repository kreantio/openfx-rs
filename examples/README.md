# Example Index

This folder contains examples for the projects in this repository.

The name of each outermost folder in this directory indicates which project the
examples are for (except for the `shared` folder, which contains content shared
across examples):

- [`layer-low`](./layer-low/): The examples demonstrate how to write OpenFX
  plugins using `openfx::{low, low_plugin}`.
  - See also: [layer-low/README.md](./layer-low/README.md)

## Examples Categorized by Topic

### Plugin Making: `official-guide-example-plugins`

These are the ports of the example plugins in the official
[OpenFX Programming Guide].

| Example Name             | Links (`layer-low`)                                                                               |
| ------------------------ | ------------------------------------------------------------------------------------------------- |
| `OFX Basics Example`     | [`plugin_1_basics.rs`](./layer-low/official-guide-example-plugins/src/plugin_1_basics.rs)         |
| `OFX Invert Example`     | [`plugin_2_invert.rs`](./layer-low/official-guide-example-plugins/src/plugin_2_invert.rs)         |
| `OFX Gain Example`       | [`plugin_3_gain.rs`](./layer-low/official-guide-example-plugins/src/plugin_3_gain.rs)             |
| `OFX Saturation Example` | [`plugin_4_saturation.rs`](./layer-low/official-guide-example-plugins/src/plugin_4_saturation.rs) |
| `OFX Circle Example`     | [`plugin_5_circle.rs`](./layer-low/official-guide-example-plugins/src/plugin_5_circle.rs)         |

### Plugin Making: `official-example-plugins`

These are the ports of the example plugins under the official repository's
[`Examples` folder].

| Example Name                                   | Links (`layer-low`)                                                             | Keywords                                                     |
| ---------------------------------------------- | ------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| Basic (`OFX Gain Example`)                     | TODO                                                                            | (TBD)                                                        |
| ChoiceParams (`OFX Choice Param Example`)      | TODO                                                                            | `kOfxParamTypeChoice`                                        |
| ColourSpace (…)                                | TODO                                                                            | (TBD)                                                        |
| Custom (`OFX Custom Param Example`)            | TODO                                                                            | `kOfxParamTypeCustom`                                        |
| DepthConverter (`OFX Depth Converter Example`) | TODO                                                                            | (TBD)                                                        |
| DrawSuite (`OFX DrawSuite Example`)            | TODO                                                                            | `OfxInteractSuiteV1`, `…OverlayInteractV2`, `OfxDrawSuiteV1` |
| Invert (`OFX Invert Example`)                  | TODO                                                                            | (TBD)                                                        |
| OpenGL (`OFX OpenGL Example`)                  | [`plugin_opengl.rs`](./layer-low/official-example-plugins/src/plugin_opengl.rs) | `OfxImageEffectOpenGLRenderSuiteV1`                          |
| Overlay (`OFX Overlay Example`)                | TODO                                                                            | `OfxInteractSuiteV1`, `…OverlayInteractV1`                   |
| Rectangle (`OFX Rectangle Example`)            | TODO                                                                            | `kOfxImageEffectContextGenerator`                            |
| Test (`OFX Test Properties`)                   | TODO                                                                            | (TBD)                                                        |

### Plugin Making: `official-old-support-example-plugins`

These are the ports of the example plugins under the official repository's
[`Support/Plugins` folder].

“old” doesn't mean that these examples are outdated. Quite the opposite: The
original versions of the examples here are more “modern” than those of the
examples in `official-example-plugins`, since they interact with a higher-level
C++ support library rather than with the primitive OpenFX C API directly. It is
just that, as of 2026, a new official C++ support library is in the process of
being introduced (under the official repository's [`openfx-cpp` folder]), and
the C++ support library these examples rely on is considered "old" compared to
the new one.

For an example ported here, apart from categorization, it doesn't matter whether
the original version uses a support library, or which support library it uses.
They are all Rust code utilizing `openfx::{low, low_plugin}` now.

| Example Name                                  | Links (`layer-low`)                                                                           | Keywords                           |
| --------------------------------------------- | --------------------------------------------------------------------------------------------- | ---------------------------------- |
| Basic (`Gain`)                                | TODO                                                                                          | (TBD)                              |
| ChoiceParams (`Choice Params (Support)`)      | TODO                                                                                          | `kOfxParamTypeChoice`              |
| Field (`Field`)                               | WON'T DO                                                                                      | [Field]                            |
| GPUGain (`GPU Gain`)                          | [plugin_gpu_gain.rs](./layer-low/official-old-support-example-plugins/src/plugin_gpu_gain.rs) | CUDA, OpenCL, Metal Buffer         |
| Generator (`Noise`)                           | TODO                                                                                          | `kOfxImageEffectContextGenerator`  |
| Invert (`Invert`)                             | TODO                                                                                          | (TBD)                              |
| MultiBundle[^1] (`Gamma` and `Dot Generator`) | TODO                                                                                          | (TBD)                              |
| Retimer (`Retimer`)                           | TODO                                                                                          | `kOfxImageEffectContextRetimer`    |
| Tester (`GenericTest`)                        | TODO                                                                                          | (TBD)                              |
| Transition (`Cross Fade`)                     | TODO                                                                                          | `kOfxImageEffectContextTransition` |
| ../PropTester (`Prop Tester`)                 | TODO                                                                                          | (TBD)                              |

### Plugin Making: `our-plugin-examples`

| Example Name | Links (`layer-low`) | Keywords |
| ------------ | ------------------- | -------- |

### Host Making: `our-host-examples`

| Example Name     | Links (`ohk`)                               | Keywords |
| ---------------- | ------------------------------------------- | -------- |
| `ofx-ls-plugins` | [README.md](./ohk/ofx-ls-plugins/README.md) |          |

[^1]: The official version of this example is intended to showcase a single
    OpenFX bundle containing multiple plugins. We already do this: example
    plugins for the same project with the same topic are bundled together into
    one bundle.

[OpenFX Programming Guide]: https://openfx.readthedocs.io/en/main/Guide/index.html
[`Examples` folder]: https://github.com/AcademySoftwareFoundation/openfx/tree/main/Examples
[`Support/Plugins` folder]: https://github.com/AcademySoftwareFoundation/openfx/tree/main/Support/Plugins
[`openfx-cpp` folder]: https://github.com/AcademySoftwareFoundation/openfx/tree/main/openfx-cpp
[Field]: https://en.wikipedia.org/wiki/Field_(video)
