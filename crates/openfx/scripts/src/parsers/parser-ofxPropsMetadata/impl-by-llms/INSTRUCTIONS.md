# Instructions for Implementing the Parser for `ofxPropsMetadata.h`

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
