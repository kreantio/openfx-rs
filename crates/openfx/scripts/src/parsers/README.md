## Why custom parsers?

Before choosing custom parsers, I evaluated:

- ~~`clang++ -ast-dump=json`~~
  - Even when only parsing the AST, `clang` requires valid input. Unfortunately,
    the current C++ headers provided by OpenFX are broken, so I would have to
    inject dummy code to make it work, which feels very hacky.
  - Even with an AST, it would be filled with noise, and I would still have to
    understand its structure to extract the information I need.
- ~~`npm:tree-sitter`~~
  - Its installation requires running build scripts. Given the JavaScript
    ecosystem's supply-chain security issues, I do not like that idea.
  - Although I have not verified it myself, it does not appear to provide
    TypeScript types for its output, so I would still have to understand the
    output structure.

Instead, I defined the types I need, wrote tests to validate them, chose a
parser library that works in plain JavaScript, and let LLMs handle the rest.

Because the problem is well-defined, the code runs only occasionally, is never
exposed to users, and is sandboxed within Deno, I consider this a good use case
for vibe coding. In other words, I did not look into the generated code here.
