# CONTRIBUTING

## LLM Policy

Code generated entirely by LLMs should live in vibe-zone directories
(`**/vibe-zone/` or `**/vibe_zone/`). This restriction does not apply to
existing code that LLMs modify on a limited, controllable scale.

Small LLM-generated code chunks, such as individual functions, may live outside
the vibe-zone directories, but must include attribution in this format:
`Author: <Harness> / <Model> (<Optional Extra Information>)`.

## Decisions

### Codegen: C headers -> `sys` layer

| Chosen? | Plan                                                                | Pros                                                  | Cons                                                                                                                          | Rationale                                                                                                     |
| ------- | ------------------------------------------------------------------- | ----------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| ✅      | Run `bindgen` on each header and deduplicate the results afterward. | Bindings for each header live in their own Rust file. | Inefficient: for example, because `ofxCore.h` is included by every other header, `bindgen` processes it once for each header. | Yes, because distinguishing headers is necessary to split the API into `generic` and `image_effect_v1` parts. |
| No      | Use an umbrella header and run `bindgen` on it once.                | Efficient.                                            | All bindings would be generated into a single Rust file.                                                                      | No, because each header should have its own Rust file.                                                        |
| No      | Write a custom parser for the C headers.                            | Efficient.                                            | Requires too much work, care, and expertise to implement correctly.                                                           | Just no.                                                                                                      |

| Chosen? | Plan                                                                     | Pros                                                                                     | Cons                                                                                                                                                                                                                 | Rationale                                                |
| ------- | ------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------- |
| ✅      | Write a project-local command-line tool and run it manually when needed. | Allows us to control when generation runs.                                               | Requires extra care to keep generated code up to date (TODO: CI).                                                                                                                                                    | Yes, because it avoids the drawbacks of the alternative. |
| No      | Use `build.rs`.                                                          | Avoids synchronization issues because the original C headers remain the source of truth. | It would run more often than necessary. Combined with the inefficient binding-generation approach, this would worsen the experience for developers who depend on this crate, especially on less performant machines. | No, because the drawbacks outweigh the benefits.         |

### Codegen: C++ headers -> layers above `sys`

| Chosen?      | Plan                                                                          | Pros                  | Cons                                                | Rationale                                                                 |
| ------------ | ----------------------------------------------------------------------------- | --------------------- | --------------------------------------------------- | ------------------------------------------------------------------------- |
| Currently ✅ | Generate Rust code in `deno`.                                                 | Quick to get started. | It feels off to use strings to construct Rust code. | Yes, because it allows rapid prototyping and iteration.                   |
| TODO         | Generate Rust code in Rust (extending the project-local CLI mentioned above). |                       |                                                     | This may be refactored in the future, but it is not currently a priority. |

| Chosen? | Plan                                            | Pros                                                                                                                                                                                                                                                                                                           | Cons                                                                                                                               | Rationale                                                                                                         |
| ------- | ----------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| ✅      | Let LLMs write a custom parser.                 | Unlike generating bindings for C headers, which requires the generator to understand types and similar constructs, the information needed from these C++ headers is textual and could likely be extracted with regular expressions. The parser here is essentially a more reliable regular-expression machine. |                                                                                                                                    | Yes. The parser can be well defined with tests, and the code is sandboxed in `deno`, making LLMs a good fit here. |
| No      | Parse the C++ headers with `clang++ -ast-dump`. |                                                                                                                                                                                                                                                                                                                | The current C++ headers provided by OpenFX are broken. To make this work, dummy code would have to be injected, which feels hacky. | No, because it is hacky.                                                                                          |
| No      | Use `npm:tree-sitter` (with `deno`).            |                                                                                                                                                                                                                                                                                                                | It requires running build scripts.                                                                                                 | No, because I do not want to run build scripts.                                                                   |

| Chosen?   | Plan                                         | Pros                                                               | Cons                                                                                                                                                                                                                     | Rationale                                                                                                    |
| --------- | -------------------------------------------- | ------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------ |
| ✅        | Generate code that calls procedural macros.  | Grammars are flexible to define. Easier to implement and maintain. | Grammars are non-standard.                                                                                                                                                                                               | I prefer them.                                                                                               |
| Partially | Generate code that calls declarative macros. | Less duplication. No code runs at compile time.                    | Declarative macros are hard to write and maintain as complexity grows. Grammars are non-standard.                                                                                                                        | Most have been replaced with procedural macros, but some will be kept or introduced when they remain simple. |
| No        | Generate regular Rust code directly.         | No indirection.                                                    | Because generated code is checked in and distributed, this would create substantial duplication in version control and distributions. The generated code would also be harder to understand because of that duplication. | Readability is still desired, even in generated code.                                                        |

## Terminology

### Terminology for Property Names

This project refers to properties in three ways:

- **Key names** (also called **k names**) are macro names defined in the OpenFX
  C headers, such as `kOfxPropAPIVersion`. They always begin with `k`.
- **Canonical names** are key names with the leading `k` removed, such as
  `OfxPropAPIVersion`.
- **Key constant values** (also called **key constants**) are the values of
  key-name macros, such as `OfxPropAPIVersion`. Most match their canonical
  names, but some are irregular. For example, the property with canonical name
  `OfxPropKeyString` has the key constant `kOfxPropKeyString` rather than
  `OfxPropKeyString`: the `k` was included in the key constant.

In this project, the `sys` layer exposes properties by their key names, while
other layers, such as the `low` layer, use their canonical names (or simplified
forms derived from those names). Because the official C++ headers use key
constants, processing must map those values back to canonical names.

#### Examples

```c
// ofxCore.h:
#define kOfxPropAPIVersion "OfxPropAPIVersion"

// ofxImageEffect.h:
#define kOfxImageEffectPropSupportsMultipleClipDepths "OfxImageEffectPropMultipleClipDepths"

// ofxKeySyms.h:
#define kOfxPropKeySym "kOfxPropKeySym"
```

| Key Name                                      | Canonical Name                               | Key Constant                         | Regular? |
| --------------------------------------------- | -------------------------------------------- | ------------------------------------ | -------- |
| kOfxPropAPIVersion                            | OfxPropAPIVersion                            | OfxPropAPIVersion                    | yes      |
| kOfxImageEffectPropSupportsMultipleClipDepths | OfxImageEffectPropSupportsMultipleClipDepths | OfxImageEffectPropMultipleClipDepths | no       |
| kOfxPropKeySym                                | OfxPropKeySym                                | kOfxPropKeySym                       | no       |

### Terminology for Property Enum Variant Names

Property enum variant names follow the same terminology as property names.

#### Examples

```c
// ofxImageEffect.h:
#define kOfxImageComponentNone "OfxImageComponentNone"

// ofxImageEffect.h:
#define kOfxImageFieldNone "OfxFieldNone"
```

| Key Name               | Canonical Name        | Key Constant          | Regular? |
| ---------------------- | --------------------- | --------------------- | -------- |
| kOfxImageComponentNone | OfxImageComponentNone | OfxImageComponentNone | yes      |
| kOfxImageFieldNone     | OfxImageFieldNone     | OfxFieldNone          | no       |

### Terminology for Action Names

Action names follow the same terminology as property names.

#### Examples

```c
// ofxCore.h:
#define  kOfxActionLoad "OfxActionLoad"
```

| Key Name       | Canonical Name | Key Constant  | Regular? |
| -------------- | -------------- | ------------- | -------- |
| kOfxActionLoad | OfxActionLoad  | OfxActionLoad | yes      |
