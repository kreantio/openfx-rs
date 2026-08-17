// Parser for `ofxPropsBySet.h`, implemented per ./INSTRUCTIONS.md.
//
// The grammar mirrors the `Structure` schema in ../types.ts one-to-one: every
// element of the expected structure corresponds to exactly one parser rule,
// and the rules appear in exactly the order the structure tuple prescribes.
// Anything the header adds, removes, or reorders therefore fails the parse.
// Unlike `ofxPropsMetadata.h`, almost every region of this header contributes
// to the result, so nearly everything is parsed structurally; only the body
// of `struct Prop` (whose contents are irrelevant) is consumed through a
// bracket-delimited "soup" rule that is still strict about its surroundings.

import { createToken, CstParser, EOF, Lexer } from "chevrotain";
import type { TokenType } from "chevrotain";
import {
  fail,
  Include,
  lexingError,
  makeHeaderLexer,
  parseCppString,
  parsingError,
  PragmaOnce,
  toInt,
} from "../../common-by-llms/mod.ts";
import type { LexIssue, ParseIssue } from "../../common-by-llms/mod.ts";
import type { FinalResult, Prop, Result, Structure } from "../types.ts";

// ---------------------------------------------------------------------------
// Tokens
// ---------------------------------------------------------------------------

// --- token categories -------------------------------------------------------
// The only ignored region is `struct Prop`'s body, so a single category
// suffices: "any token except a brace" may appear inside it. Structural
// keywords and fixed names (e.g. `namespace`, `prop_sets`) deliberately carry
// no category, so their appearance inside the body is a parse error.

const AnyExceptBrace = createToken({
  name: "AnyExceptBrace",
  pattern: Lexer.NA,
});

// --- keywords ---------------------------------------------------------------

const NamespaceKW = createToken({
  name: "NamespaceKW",
  pattern: /namespace\b/,
});
const StructKW = createToken({ name: "StructKW", pattern: /struct\b/ });
const StaticAssertKW = createToken({
  name: "StaticAssertKW",
  pattern: /static_assert\b/,
});
const StaticKW = createToken({ name: "StaticKW", pattern: /static\b/ });
const InlineKW = createToken({ name: "InlineKW", pattern: /inline\b/ });
// `const` may appear inside the ignored struct body.
const ConstKW = createToken({
  name: "ConstKW",
  pattern: /const\b/,
  categories: [AnyExceptBrace],
});

// --- fixed names ------------------------------------------------------------

const StdStringView = createToken({
  name: "StdStringView",
  pattern: /std::string_view\b/,
});
const StdMap = createToken({ name: "StdMap", pattern: /std::map\b/ });
const StdVector = createToken({ name: "StdVector", pattern: /std::vector\b/ });
const StdArray = createToken({ name: "StdArray", pattern: /std::array\b/ });
// `::` must lex after the `std::…` compounds but before the bare `:`.
const ScopeRes = createToken({ name: "ScopeRes", pattern: /::/ });
const PropDefsVar = createToken({
  name: "PropDefsVar",
  pattern: /prop_defs\b/,
});
const PropSetsVar = createToken({
  name: "PropSetsVar",
  pattern: /prop_sets\b/,
});
const ActionsVar = createToken({ name: "ActionsVar", pattern: /actions\b/ });
const ActionPropsVar = createToken({
  name: "ActionPropsVar",
  pattern: /action_props\b/,
});
const PropIdKW = createToken({ name: "PropIdKW", pattern: /PropId\b/ });
const OpenfxNS = createToken({ name: "OpenfxNS", pattern: /openfx\b/ });
const TrueKW = createToken({ name: "TrueKW", pattern: /true\b/ });
const FalseKW = createToken({ name: "FalseKW", pattern: /false\b/ });

// --- punctuation and literals ------------------------------------------------

const LBrace = createToken({ name: "LBrace", pattern: /\{/ });
const RBrace = createToken({ name: "RBrace", pattern: /\}/ });
// These may all appear inside the ignored struct body (constructor signature).
const LParen = createToken({
  name: "LParen",
  pattern: /\(/,
  categories: [AnyExceptBrace],
});
const RParen = createToken({
  name: "RParen",
  pattern: /\)/,
  categories: [AnyExceptBrace],
});
const LBracket = createToken({ name: "LBracket", pattern: /\[/ });
const RBracket = createToken({ name: "RBracket", pattern: /\]/ });
const Less = createToken({ name: "Less", pattern: /</ });
const Greater = createToken({ name: "Greater", pattern: />/ });
const Semicolon = createToken({
  name: "Semicolon",
  pattern: /;/,
  categories: [AnyExceptBrace],
});
const Comma = createToken({
  name: "Comma",
  pattern: /,/,
  categories: [AnyExceptBrace],
});
const Colon = createToken({
  name: "Colon",
  pattern: /:/,
  categories: [AnyExceptBrace],
});
const Star = createToken({
  name: "Star",
  pattern: /\*/,
  categories: [AnyExceptBrace],
});
const Amp = createToken({
  name: "Amp",
  pattern: /&/,
  categories: [AnyExceptBrace],
});
const EqEq = createToken({ name: "EqEq", pattern: /==/ });
const StringLiteral = createToken({
  name: "StringLiteral",
  pattern: /"(?:[^"\\\n\r]|\\.)*"/,
});
const NumberLiteral = createToken({ name: "NumberLiteral", pattern: /\d+/ });
const Identifier = createToken({
  name: "Identifier",
  pattern: /[A-Za-z_][A-Za-z0-9_]*/,
  categories: [AnyExceptBrace],
});

// Longer/compound patterns must precede their prefixes and `Identifier`.
const lexerTokens: TokenType[] = [
  StaticAssertKW,
  EqEq,
  StdStringView,
  StdMap,
  StdVector,
  StdArray,
  ScopeRes,
  NamespaceKW,
  StructKW,
  StaticKW,
  InlineKW,
  ConstKW,
  PropDefsVar,
  PropSetsVar,
  ActionsVar,
  ActionPropsVar,
  PropIdKW,
  OpenfxNS,
  TrueKW,
  FalseKW,
  LBrace,
  RBrace,
  LParen,
  RParen,
  LBracket,
  RBracket,
  Less,
  Greater,
  Semicolon,
  Comma,
  Colon,
  Star,
  Amp,
  StringLiteral,
  NumberLiteral,
  Identifier,
];
const parserTokens: TokenType[] = [
  ...lexerTokens,
  PragmaOnce,
  Include,
  AnyExceptBrace,
];

const ofxLexer = makeHeaderLexer(lexerTokens);

// ---------------------------------------------------------------------------
// Parser
// ---------------------------------------------------------------------------

type OpenfxInner = Structure[2]["inner"];

class OfxPropsBySetParser extends CstParser {
  // --- collected data ---------------------------------------------------------

  readonly propSets: Record<string, Record<string, Prop>> = {};
  readonly actions: string[] = [];
  readonly actionProps: Record<
    string,
    { inArgs?: Set<string>; outArgs?: Set<string> }
  > = {};

  // --- structure markers and cross-check state --------------------------------

  readonly openfxInner: string[] = [];
  /** Name of the prop set whose entries are currently being parsed. */
  private currentSet: string | undefined;
  /** Last set name, for the strictly-ascending check. */
  private lastSetName: string | undefined;
  /** Last action_props key, for the strictly-ascending check. */
  private lastActionPropKey: string | undefined;
  /** Result of the most recent `boolLiteral` subrule. */
  private boolValue = false;
  /** Index into `actions` past the last static_assert match (order check). */
  private assertCursor = 0;
  private assertedNames = new Set<string>();

  constructor() {
    super(parserTokens);
    this.performSelfAnalysis();
  }

  // header: `#pragma once`, includes, then the single `openfx` namespace.
  headerFile = this.RULE("headerFile", () => {
    this.CONSUME(PragmaOnce);
    this.AT_LEAST_ONE(() => this.CONSUME(Include));
    this.SUBRULE(this.namespaceOpenfx);
    this.CONSUME(EOF);
  });

  // namespace openfx { struct; prop_sets; actions; action_props; asserts; }
  namespaceOpenfx = this.RULE("namespaceOpenfx", () => {
    this.CONSUME(NamespaceKW);
    this.CONSUME(OpenfxNS);
    this.CONSUME(LBrace);
    this.SUBRULE(this.structProp);
    this.SUBRULE(this.staticPropSets);
    this.SUBRULE(this.staticActions);
    this.SUBRULE(this.staticActionProps);
    this.AT_LEAST_ONE(() => this.SUBRULE(this.staticAssertDeclaration));
    this.ACTION(() => this.openfxInner.push("staticAssert[]"));
    this.CONSUME(RBrace);
  });

  // `struct Prop { ... };` -- body is irrelevant to the result.
  structProp = this.RULE("structProp", () => {
    this.CONSUME(StructKW);
    const name = this.CONSUME(Identifier);
    this.ACTION(() => {
      if (name.image !== "Prop") {
        fail(`expected struct name "Prop" but got "${name.image}"`);
      }
    });
    this.SUBRULE(this.braceBlock);
    this.CONSUME(Semicolon);
    this.ACTION(() => this.openfxInner.push("struct:prop"));
  });

  // A `{ ... }` block with balanced nested blocks, contents unconstrained
  // (but only category-member tokens are allowed inside).
  braceBlock = this.RULE("braceBlock", () => {
    this.CONSUME(LBrace);
    this.MANY(() => {
      this.OR([
        { ALT: () => this.CONSUME(AnyExceptBrace) },
        { ALT: () => this.SUBRULE(this.braceBlock) },
      ]);
    });
    this.CONSUME(RBrace);
  });

  // static inline const std::map<const char *, std::vector<Prop>> prop_sets
  //   { { "Set", { entry, ..., entry } }, ... };
  staticPropSets = this.RULE("staticPropSets", () => {
    this.CONSUME(StaticKW);
    this.CONSUME(InlineKW);
    this.CONSUME(ConstKW);
    this.CONSUME(StdMap);
    this.CONSUME(Less);
    this.CONSUME2(ConstKW);
    const keyElem = this.CONSUME(Identifier);
    this.CONSUME(Star);
    this.CONSUME(Comma);
    this.CONSUME(StdVector);
    this.CONSUME2(Less);
    const valueElem = this.CONSUME2(Identifier);
    this.CONSUME(Greater);
    this.CONSUME2(Greater);
    this.CONSUME(PropSetsVar);
    this.ACTION(() => {
      if (keyElem.image !== "char") {
        fail(`unexpected prop_sets key type "${keyElem.image}"`);
      }
      if (valueElem.image !== "Prop") {
        fail(`unexpected prop_sets value type "${valueElem.image}"`);
      }
    });
    this.CONSUME(LBrace);
    this.AT_LEAST_ONE(() => this.SUBRULE(this.propSetPair));
    this.CONSUME(RBrace);
    this.CONSUME(Semicolon);
    this.ACTION(() => this.openfxInner.push("static:propSets"));
  });

  // { "SetName", { entry, ..., entry } }, -- trailing comma on every pair;
  // entries inside the vector carry commas between them but not after the
  // last one.
  propSetPair = this.RULE("propSetPair", () => {
    this.CONSUME(LBrace);
    const nameTok = this.CONSUME(StringLiteral);
    this.CONSUME(Comma);
    this.CONSUME2(LBrace);
    this.ACTION(() => {
      const name = parseCppString(nameTok.image);
      if (this.propSets[name] !== undefined) {
        fail(`duplicate prop_sets entry "${name}"`);
      }
      if (this.lastSetName !== undefined && !(this.lastSetName < name)) {
        fail(
          `prop_sets entry "${name}" does not follow "${this.lastSetName}" ` +
            `in ascending order`,
        );
      }
      this.lastSetName = name;
      this.currentSet = name;
      this.propSets[name] = {};
    });
    this.SUBRULE(this.propEntry);
    this.MANY(() => {
      this.CONSUME2(Comma);
      this.SUBRULE2(this.propEntry);
    });
    this.CONSUME(RBrace);
    this.CONSUME2(RBrace);
    this.CONSUME3(Comma);
    this.ACTION(() => {
      this.currentSet = undefined;
    });
  });

  // { "Name", prop_defs[PropId::Id], hostWrite, pluginWrite, hostOptional }
  propEntry = this.RULE("propEntry", () => {
    this.CONSUME(LBrace);
    const nameTok = this.CONSUME(StringLiteral);
    this.CONSUME(Comma);
    this.CONSUME(PropDefsVar);
    this.CONSUME(LBracket);
    this.CONSUME(PropIdKW);
    this.CONSUME(ScopeRes);
    const idTok = this.CONSUME(Identifier);
    this.CONSUME(RBracket);
    this.CONSUME2(Comma);
    this.SUBRULE(this.boolLiteral);
    const hostWrite = this.boolValue;
    this.CONSUME3(Comma);
    this.SUBRULE2(this.boolLiteral);
    const pluginWrite = this.boolValue;
    this.CONSUME4(Comma);
    this.SUBRULE3(this.boolLiteral);
    const hostOptional = this.boolValue;
    this.CONSUME(RBrace);
    this.ACTION(() => {
      const name = parseCppString(nameTok.image);
      const id = idTok.image;
      // The prop name is the PropId member, optionally `k`-prefixed.
      if (name !== id && name !== `k${id}`) {
        fail(`prop entry "${name}" references unexpected PropId "${id}"`);
      }
      const set = this.currentSet;
      if (set === undefined) {
        fail(`prop entry "${name}" appears outside a prop set`);
      }
      // Duplicate names within a set are allowed (the record keeps the last).
      this.propSets[set]![name] = {
        propDef: id,
        hostWrite,
        pluginWrite,
        hostOptional,
      };
    });
  });

  boolLiteral = this.RULE("boolLiteral", () => {
    this.boolValue = false;
    this.OR([
      {
        ALT: () => {
          this.CONSUME(TrueKW);
          this.boolValue = true;
        },
      },
      { ALT: () => this.CONSUME(FalseKW) },
    ]);
  });

  // static inline const std::array<const char *, N> actions { "...", ... };
  // every element (including the last) is followed by a comma.
  staticActions = this.RULE("staticActions", () => {
    this.CONSUME(StaticKW);
    this.CONSUME(InlineKW);
    this.CONSUME(ConstKW);
    this.CONSUME(StdArray);
    this.CONSUME(Less);
    this.CONSUME2(ConstKW);
    const elemType = this.CONSUME(Identifier);
    this.CONSUME(Star);
    this.CONSUME(Comma);
    const countTok = this.CONSUME(NumberLiteral);
    this.CONSUME(Greater);
    this.CONSUME(ActionsVar);
    this.CONSUME(LBrace);
    const names: string[] = [];
    const first = this.CONSUME(StringLiteral);
    this.ACTION(() => names.push(parseCppString(first.image)));
    this.MANY(() => {
      this.CONSUME2(Comma);
      const next = this.CONSUME2(StringLiteral);
      this.ACTION(() => names.push(parseCppString(next.image)));
    });
    this.CONSUME3(Comma);
    this.CONSUME(RBrace);
    this.CONSUME(Semicolon);
    this.ACTION(() => {
      if (elemType.image !== "char") {
        fail(`unexpected actions element type "${elemType.image}"`);
      }
      const declared = toInt(countTok.image, "actions element count");
      if (declared !== names.length) {
        fail(
          `actions declares ${declared} element(s) but lists ${names.length}`,
        );
      }
      for (let i = 1; i < names.length; i++) {
        if (!(names[i - 1]! < names[i]!)) {
          fail(
            `actions name "${names[i]}" does not follow "${names[i - 1]}" ` +
              `in ascending order`,
          );
        }
      }
      this.actions.push(...names);
      this.openfxInner.push("static:actions");
    });
  });

  // static inline const
  //   std::map<std::array<std::string_view, 2>, std::vector<const char *>>
  //   action_props { { { "Action", kind }, { "Value", ... } }, ... };
  staticActionProps = this.RULE("staticActionProps", () => {
    this.CONSUME(StaticKW);
    this.CONSUME(InlineKW);
    this.CONSUME(ConstKW);
    this.CONSUME(StdMap);
    this.CONSUME(Less);
    this.CONSUME(StdArray);
    this.CONSUME2(Less);
    this.CONSUME(StdStringView);
    this.CONSUME(Comma);
    const keyArity = this.CONSUME(NumberLiteral);
    this.CONSUME(Greater);
    this.CONSUME2(Comma);
    this.CONSUME(StdVector);
    this.CONSUME3(Less);
    this.CONSUME2(ConstKW);
    const elemType = this.CONSUME(Identifier);
    this.CONSUME(Star);
    this.CONSUME2(Greater);
    this.CONSUME3(Greater);
    this.CONSUME(ActionPropsVar);
    this.ACTION(() => {
      // The key is an (action, kind) pair; the pair shape requires arity 2.
      if (toInt(keyArity.image, "action_props key arity") !== 2) {
        fail(`unexpected action_props key arity "${keyArity.image}"`);
      }
      if (elemType.image !== "char") {
        fail(`unexpected action_props element type "${elemType.image}"`);
      }
    });
    this.CONSUME(LBrace);
    this.AT_LEAST_ONE(() => this.SUBRULE(this.actionPropsEntry));
    this.CONSUME(RBrace);
    this.CONSUME(Semicolon);
    this.ACTION(() => this.openfxInner.push("static:actionProps"));
  });

  // { { "Action", inArgs|outArgs }, { "Value", ..., "Value" } },
  // (trailing comma on every entry).
  actionPropsEntry = this.RULE("actionPropsEntry", () => {
    this.CONSUME(LBrace);
    this.CONSUME2(LBrace);
    const actionTok = this.CONSUME(StringLiteral);
    this.CONSUME(Comma);
    const kindTok = this.CONSUME2(StringLiteral);
    this.ACTION(() => {
      const kind = parseCppString(kindTok.image);
      if (kind !== "inArgs" && kind !== "outArgs") {
        fail(`unexpected action_props key kind "${kind}"`);
      }
    });
    this.CONSUME(RBrace);
    this.CONSUME2(Comma);
    this.CONSUME3(LBrace);
    const values: string[] = [];
    const first = this.CONSUME3(StringLiteral);
    this.ACTION(() => values.push(parseCppString(first.image)));
    this.MANY(() => {
      this.CONSUME3(Comma);
      const next = this.CONSUME4(StringLiteral);
      this.ACTION(() => values.push(parseCppString(next.image)));
    });
    this.CONSUME2(RBrace);
    this.CONSUME3(RBrace);
    this.CONSUME4(Comma);
    this.ACTION(() => {
      const action = parseCppString(actionTok.image);
      const kindName = parseCppString(kindTok.image) as "inArgs" | "outArgs";
      const key = JSON.stringify([action, kindName]);
      if (
        this.lastActionPropKey !== undefined && !(this.lastActionPropKey < key)
      ) {
        fail(
          `action_props entry ${key} does not follow ${this.lastActionPropKey}` +
            ` in ascending order`,
        );
      }
      this.lastActionPropKey = key;
      if (!this.actions.includes(action)) {
        fail(`action_props references "${action}", which is not in actions`);
      }
      for (let i = 1; i < values.length; i++) {
        if (values[i - 1]! > values[i]!) {
          fail(
            `action_props ${key}: value "${values[i]}" does not follow ` +
              `"${values[i - 1]}" in non-decreasing order`,
          );
        }
      }
      const entry = this.actionProps[action] ??= {};
      if (entry[kindName] !== undefined) {
        fail(`duplicate action_props entry ${key}`);
      }
      entry[kindName] = new Set(values);
    });
  });

  // static_assert(std::string_view("Name") == std::string_view(kName));
  staticAssertDeclaration = this.RULE("staticAssertDeclaration", () => {
    this.CONSUME(StaticAssertKW);
    this.CONSUME(LParen);
    this.CONSUME(StdStringView);
    this.CONSUME2(LParen);
    const nameTok = this.CONSUME(StringLiteral);
    this.CONSUME(RParen);
    this.CONSUME(EqEq);
    this.CONSUME2(StdStringView);
    this.CONSUME3(LParen);
    const identTok = this.CONSUME(Identifier);
    this.CONSUME2(RParen);
    this.CONSUME3(RParen);
    this.CONSUME(Semicolon);
    this.ACTION(() => {
      const name = parseCppString(nameTok.image);
      if (identTok.image !== `k${name}`) {
        fail(`static_assert compares "${name}" with "${identTok.image}"`);
      }
      if (this.assertedNames.has(name)) {
        fail(`duplicate static_assert for "${name}"`);
      }
      this.assertedNames.add(name);
      // Asserts must appear in the same relative order as `actions`.
      const idx = this.actions.indexOf(name, this.assertCursor);
      if (idx === -1) {
        fail(`static_assert references "${name}", which is not in actions`);
      }
      this.assertCursor = idx + 1;
    });
  });
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

const HEADER_NAME = "ofxPropsBySet.h";

export function parse(headerCode: string): Result {
  const lexingResult = ofxLexer.tokenize(headerCode);
  if (lexingResult.errors.length > 0) {
    lexingError(HEADER_NAME, headerCode, lexingResult.errors as LexIssue[]);
  }

  const parser = new OfxPropsBySetParser();
  parser.input = lexingResult.tokens;
  parser.headerFile();
  if (parser.errors.length > 0) {
    parsingError(HEADER_NAME, headerCode, parser.errors as ParseIssue[]);
  }

  return {
    structure: [
      "pragmaOnce",
      "include[]",
      {
        type: "namespace:openfx",
        // The grammar pushes exactly the elements the Structure schema
        // prescribes, in order; the cast only fixes up the tuple type.
        inner: [...parser.openfxInner] as unknown as OpenfxInner,
      },
    ],
    infos: {
      propSets: parser.propSets,
      actions: new Set(parser.actions),
      actionProps: parser.actionProps,
    },
  };
}

export function makeFinalResult(result: Result): FinalResult {
  return { infos: result.infos };
}
