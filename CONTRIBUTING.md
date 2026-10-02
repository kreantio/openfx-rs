# CONTRIBUTING

## AI Policy

See [AI_POLICY.md](./AI_POLICY.md).

## Decisions

### Codegen: C headers -> data -> `sys` layer

#### How do we generate bindings for Rust

| Chosen?                 | Plan                                                                | Pros                                        | Cons                                                                                                                                 | Rationale                                                                                                |
| ----------------------- | ------------------------------------------------------------------- | ------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| ✅ ([`openfx-datagen`]) | Write a custom parser + bindings generator for the C headers.       | Efficient. Flexible. Per-header separation. | Requires more care to maintain.                                                                                                      | Yes. Implementing the first working version required much work, but maintenance afterward is manageable. |
| Was                     | Run `bindgen` on each header and deduplicate the results afterward. | Per-header separation.                      | Inefficient: for example, because `ofxCore.h` is included by every other header, `bindgen` processes it once for each header. Hacky. | Moved away from, because the implementation was hacky and not flexible enough.                           |
| No                      | Use an umbrella header and run `bindgen` on it once.                | Efficient.                                  | All bindings would be generated into a single Rust file.                                                                             | No, because each header should have its own Rust file.                                                   |

#### When do we generate these bindings

| Chosen? | Plan                                                                                         | Pros                                                                                                       | Cons                                                              | Rationale                                                             |
| ------- | -------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------- | --------------------------------------------------------------------- |
| ✅      | Write CLI tools (`openfx-codegen` and [`openfx-datagen`]) and run them manually when needed. | Allows us to control when generation runs. The `openfx` crate is free of dependencies for code generation. | Requires extra care to keep generated code up to date (TODO: CI). | This is the established approach, and we have no reason to change it. |
| No      | Use `build.rs`.                                                                              | Avoids synchronization issues.                                                                             | It would run more often than necessary.                           |                                                                       |

#### Where to store generated data

| Chosen?              | Plan                   | Pros                                    | Cons                                                                                     | Rationale                           |
| -------------------- | ---------------------- | --------------------------------------- | ---------------------------------------------------------------------------------------- | ----------------------------------- |
| ✅ ([`openfx-data`]) | in another repository  | The main repository's size stays small. | [`openfx-datagen`] has to be in another repository to avoid a circular dependency.       | The main repository's size matters. |
| No                   | in the main repository | less friction                           | The repository's history would be filled with generated data that is no longer relevant. |                                     |

### Codegen: C++ headers -> layers above `sys`

#### Which language do we use to generate code

| Chosen?      | Plan                                                     | Pros                  | Cons                                                | Rationale                                                                 |
| ------------ | -------------------------------------------------------- | --------------------- | --------------------------------------------------- | ------------------------------------------------------------------------- |
| Currently ✅ | Generate Rust code in `deno`.                            | Quick to get started. | It feels off to use strings to construct Rust code. | Yes, because it allows rapid prototyping and iteration.                   |
| TODO         | Generate Rust code in Rust (extending `openfx-codegen`). |                       |                                                     | This may be refactored in the future, but it is not currently a priority. |

#### How do we parse the C++ headers

| Chosen?      | Plan                                                     | Pros                                                                                                                                                                                                                                                                                                           | Cons                                                                                                                               | Rationale                                                                                                         |
| ------------ | -------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| Currently ✅ | Let LLMs write a custom parser.                          | Unlike generating bindings for C headers, which requires the generator to understand types and similar constructs, the information needed from these C++ headers is textual and could likely be extracted with regular expressions. The parser here is essentially a more reliable regular-expression machine. | It violates the planned AI policy.                                                                                                 | Yes. The parser can be well defined with tests, and the code is sandboxed in `deno`, making LLMs a good fit here. |
| No           | Parse the C++ headers with `clang++ -ast-dump`.          |                                                                                                                                                                                                                                                                                                                | The current C++ headers provided by OpenFX are broken. To make this work, dummy code would have to be injected, which feels hacky. | No, because it is hacky.                                                                                          |
| No           | Use `npm:tree-sitter` (with `deno`).                     |                                                                                                                                                                                                                                                                                                                | It requires running build scripts.                                                                                                 | No, because I do not want to run build scripts.                                                                   |
| TODO         | Parse comments in the C headers with [`openfx-datagen`]. |                                                                                                                                                                                                                                                                                                                |                                                                                                                                    |                                                                                                                   |

#### How do we present code

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

[`openfx-datagen`]: https://github.com/kreantio/openfx-datagen
[`openfx-data`]: https://github.com/kreantio/openfx-data
