# Instructions for the Parser for `ofxPropsMetadata.h`

## Instructions for update 1

Update the parser so that `prop_type_array` supports multiple types.

Refer to:
`git diff crates/openfx/scripts/src/parsers/parser-ofxPropsMetadata/types.ts`.

For example, `finalResult.propertyInfos["OfxParamPropDefault"].type` must equal
`new Set(["Int", "Double", "String", "Pointer"])`.

If a property's value can have an enum type, it must not have any other type.
Throw an error if this condition is encountered.

## History

### Instructions for Implementing the Parser

Executed by `GLM 5.3 (harness: Hermes, provider: OpenCode Go, effort: Med)`.

1. Only write code in this folder.
   - Because `deno.json` is outside this folder, do not introduce new
     dependencies.
2. Use `Chevrotain` for parsing. Regexes are permitted only for token patterns;
   do not overuse them.
3. Parse only the syntax required to produce the expected output.
4. Other than the required syntax, the implementation must not depend on the
   contents of `ofxPropsMetadata.h`, including assumptions about how the file
   may evolve. Make the parser as strict as possible so that tests detect
   structural changes in the header file. This strictness applies to parsing,
   not to the implementation: keep the parser easy to adapt to future changes.
5. Acceptance criterion: `just test`, run in this folder, must pass all tests.
