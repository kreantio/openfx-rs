// Parser for `ofxPropsMetadata.h`, implemented per ./INSTRUCTIONS.md.
//
// The grammar mirrors the `Structure` schema in ../types.ts one-to-one: every
// element of the expected structure corresponds to exactly one parser rule,
// and the rules appear in exactly the order the structure tuple prescribes.
// Anything the header adds, removes, or reorders therefore fails the parse.
// Regions whose contents do not contribute to the result (struct bodies,
// template parameter lists, static_assert expressions) are consumed through
// token-category "soup" rules that are still bracket-delimited and strict
// about the surrounding shape.

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
import { PropType as PropTypeSchema } from "../types.ts";
import type { FinalResult, PropType, Result, Structure } from "../types.ts";

/** The canonical PropType set, taken from ../types.ts (single source of truth). */
const PROP_TYPES = new Set<string>(PropTypeSchema.options);

// ---------------------------------------------------------------------------
// Tokens
// ---------------------------------------------------------------------------

// --- skipped by the lexer ---------------------------------------------------
// Trivia tokens live in ../../common-by-llms (shared with the other header
// parsers) and are re-exported here only as needed by the token lists below.

// --- token categories -------------------------------------------------------
// Categories let the grammar consume "any token except X" inside regions whose
// exact contents do not contribute to the result. Membership is deliberately
// narrow: tokens that would indicate a structural surprise (e.g. a `namespace`
// keyword inside a struct body) are not members of any category, so their
// appearance there is a parse error.

const AnyExceptBrace = createToken({
  name: "AnyExceptBrace",
  pattern: Lexer.NA,
});
const AnyExceptParenOrSemicolon = createToken({
  name: "AnyExceptParenOrSemicolon",
  pattern: Lexer.NA,
});
const AnyExceptAngleOrComma = createToken({
  name: "AnyExceptAngleOrComma",
  pattern: Lexer.NA,
});

const soupEverywhere = [
  AnyExceptBrace,
  AnyExceptParenOrSemicolon,
  AnyExceptAngleOrComma,
];
const soupBraceAndParen = [AnyExceptBrace, AnyExceptParenOrSemicolon];
const soupBraceAndTemplate = [AnyExceptBrace, AnyExceptAngleOrComma];

// --- preprocessor -----------------------------------------------------------

// `#pragma once`, includes and trivia come from ../../common-by-llms. This
// dialect additionally allows a whole `#define ...` directive including
// backslash-newline continuations (a newline may only appear escaped).
const DefineDirective = createToken({
  name: "DefineDirective",
  pattern: /#define(?:[^\\\n\r]|\\[\s\S])*/,
});

// --- keywords ---------------------------------------------------------------
// Keywords that may legitimately appear inside skipped regions carry the
// corresponding soup categories; the rest do not.

const NamespaceKW = createToken({
  name: "NamespaceKW",
  pattern: /namespace\b/,
});
const EnumKW = createToken({ name: "EnumKW", pattern: /enum\b/ });
const ClassKW = createToken({ name: "ClassKW", pattern: /class\b/ });
const StructKW = createToken({ name: "StructKW", pattern: /struct\b/ });
const UsingKW = createToken({ name: "UsingKW", pattern: /using\b/ });
const StaticAssertKW = createToken({
  name: "StaticAssertKW",
  pattern: /static_assert\b/,
});
const StaticKW = createToken({
  name: "StaticKW",
  pattern: /static\b/,
  categories: soupEverywhere,
});
const InlineKW = createToken({
  name: "InlineKW",
  pattern: /inline\b/,
  categories: soupEverywhere,
});
const ConstexprKW = createToken({
  name: "ConstexprKW",
  pattern: /constexpr\b/,
  categories: soupEverywhere,
});
const ConstKW = createToken({
  name: "ConstKW",
  pattern: /const\b/,
  categories: soupEverywhere,
});
const TemplateKW = createToken({
  name: "TemplateKW",
  pattern: /template\b/,
  categories: soupEverywhere,
});
const TypenameKW = createToken({
  name: "TypenameKW",
  pattern: /typename\b/,
  categories: soupEverywhere,
});

// --- fixed multi-word symbols ------------------------------------------------
// `PropType` may appear inside skipped regions (e.g. struct fields), so it
// carries soup categories too; the namespace/variable names do not.

const OpenfxSpan = createToken({
  name: "OpenfxSpan",
  pattern: /openfx::span\b/,
  categories: soupEverywhere,
});
const StdArray = createToken({
  name: "StdArray",
  pattern: /std::array\b/,
  categories: soupEverywhere,
});
const PropTypeKW = createToken({
  name: "PropTypeKW",
  pattern: /PropType\b/,
  categories: soupEverywhere,
});
const OpenfxNS = createToken({ name: "OpenfxNS", pattern: /openfx\b/ });
const PropEnumValuesNS = createToken({
  name: "PropEnumValuesNS",
  pattern: /prop_enum_values\b/,
});
const PropTypeArraysNS = createToken({
  name: "PropTypeArraysNS",
  pattern: /prop_type_arrays\b/,
});
const PropertiesNS = createToken({
  name: "PropertiesNS",
  pattern: /properties\b/,
});
const AssertionsNS = createToken({
  name: "AssertionsNS",
  pattern: /assertions\b/,
});
const PropDefsVar = createToken({
  name: "PropDefsVar",
  pattern: /prop_defs\b/,
});
const DefinePropTraitsKW = createToken({
  name: "DefinePropTraitsKW",
  pattern: /DEFINE_PROP_TRAITS\b/,
});

// --- punctuation and literals ------------------------------------------------

const LBrace = createToken({ name: "LBrace", pattern: /\{/ });
const RBrace = createToken({ name: "RBrace", pattern: /\}/ });
const LParen = createToken({
  name: "LParen",
  pattern: /\(/,
  categories: soupBraceAndTemplate,
});
const RParen = createToken({
  name: "RParen",
  pattern: /\)/,
  categories: soupBraceAndTemplate,
});
const LBracket = createToken({
  name: "LBracket",
  pattern: /\[/,
  categories: soupEverywhere,
});
const RBracket = createToken({
  name: "RBracket",
  pattern: /\]/,
  categories: soupEverywhere,
});
const Less = createToken({
  name: "Less",
  pattern: /</,
  categories: soupBraceAndParen,
});
const Greater = createToken({
  name: "Greater",
  pattern: />/,
  categories: soupBraceAndParen,
});
const Semicolon = createToken({
  name: "Semicolon",
  pattern: /;/,
  categories: soupBraceAndTemplate,
});
const Comma = createToken({
  name: "Comma",
  pattern: /,/,
  categories: soupBraceAndParen,
});
const Equals = createToken({
  name: "Equals",
  pattern: /=/,
  categories: soupEverywhere,
});
const EqEq = createToken({
  name: "EqEq",
  pattern: /==/,
  categories: soupEverywhere,
});
const Star = createToken({
  name: "Star",
  pattern: /\*/,
  categories: soupEverywhere,
});
const Amp = createToken({
  name: "Amp",
  pattern: /&/,
  categories: soupEverywhere,
});
const Dot = createToken({
  name: "Dot",
  pattern: /\./,
  categories: soupEverywhere,
});
const ScopeRes = createToken({
  name: "ScopeRes",
  pattern: /::/,
  categories: soupEverywhere,
});
const StringLiteral = createToken({
  name: "StringLiteral",
  pattern: /"(?:[^"\\\n\r]|\\.)*"/,
  categories: soupEverywhere,
});
const NumberLiteral = createToken({
  name: "NumberLiteral",
  pattern: /\d+/,
  categories: soupEverywhere,
});
const Identifier = createToken({
  name: "Identifier",
  pattern: /[A-Za-z_][A-Za-z0-9_]*/,
  categories: soupEverywhere,
});

const dialectTokens: TokenType[] = [
  // `#define` is specific to this dialect (the DEFINE_PROP_TRAITS macro).
  DefineDirective,
  // Longer keywords before shorter prefixes and before Identifier.
  StaticAssertKW,
  EqEq,
  ScopeRes,
  ConstexprKW,
  NamespaceKW,
  EnumKW,
  ClassKW,
  StructKW,
  StaticKW,
  InlineKW,
  ConstKW,
  UsingKW,
  TemplateKW,
  TypenameKW,
  OpenfxSpan,
  StdArray,
  PropTypeKW,
  OpenfxNS,
  PropEnumValuesNS,
  PropTypeArraysNS,
  PropertiesNS,
  AssertionsNS,
  PropDefsVar,
  DefinePropTraitsKW,
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
  Equals,
  Star,
  Amp,
  Dot,
  StringLiteral,
  NumberLiteral,
  Identifier,
];
const parserTokens = [
  ...dialectTokens,
  PragmaOnce,
  Include,
  AnyExceptBrace,
  AnyExceptParenOrSemicolon,
  AnyExceptAngleOrComma,
];

const ofxLexer = makeHeaderLexer(dialectTokens);

// ---------------------------------------------------------------------------
// Parser
// ---------------------------------------------------------------------------

type OpenfxInner = Structure[2]["inner"];
type OpenfxInnerElement = OpenfxInner[number];
type PropertiesInner = Extract<
  OpenfxInnerElement,
  { type: "namespace:properties" }
>["inner"];
type AssertionsInner = Extract<
  OpenfxInnerElement,
  { type: "namespace:assertions" }
>["inner"];

class OfxPropsMetadataParser extends CstParser {
  // --- collected data ---------------------------------------------------------

  readonly propEnumValues: Record<string, Set<string>> = {};
  readonly propTypeArrays: Record<string, Set<PropType>> = {};
  readonly propDefsArray: Record<string, { dimension: number }[]> = {};
  /** Ids of DEFINE_PROP_TRAITS calls, in header order. */
  readonly propTraitsCallIds: string[] = [];
  /** Members of the `enum class PropId` (excluding the trailing NProps). */
  readonly propIdNames: string[] = [];

  // --- structure markers ------------------------------------------------------
  // Pushed by each section rule, in exactly the order the grammar enforces.

  readonly openfxInner: OpenfxInnerElement[] = [];
  readonly propertiesInner: PropertiesInner[number][] = [];
  readonly assertionsInner: AssertionsInner[number][] = [];

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

  namespaceOpenfx = this.RULE("namespaceOpenfx", () => {
    this.CONSUME(NamespaceKW);
    this.CONSUME(OpenfxNS);
    this.CONSUME(LBrace);
    this.SUBRULE(this.enumPropType);
    this.SUBRULE(this.enumPropId);
    this.SUBRULE(this.namespacePropEnumValues);
    this.SUBRULE(this.namespacePropTypeArrays);
    this.SUBRULE(this.structPropDef);
    this.SUBRULE(this.templateStructPropDefsArray);
    this.SUBRULE(this.staticPropDefs);
    this.SUBRULE(this.namespaceProperties);
    this.SUBRULE(this.namespaceAssertions);
    this.CONSUME(RBrace);
  });

  // `enum class PropType { Int, Double, Enum, Bool, String, Pointer };`
  enumPropType = this.RULE("enumPropType", () => {
    this.CONSUME(EnumKW);
    this.CONSUME(ClassKW);
    this.CONSUME(PropTypeKW);
    this.CONSUME(LBrace);
    const enumerators: string[] = [];
    const first = this.CONSUME(Identifier);
    this.ACTION(() => enumerators.push(first.image));
    this.MANY(() => {
      this.CONSUME(Comma);
      const next = this.CONSUME2(Identifier);
      this.ACTION(() => enumerators.push(next.image));
    });
    this.CONSUME(RBrace);
    this.CONSUME(Semicolon);
    this.ACTION(() => {
      // The grammar rule only sees identifiers; validate that the enumerator
      // set is exactly the canonical PropType set (order-insensitive).
      const actual = new Set(enumerators);
      if (
        actual.size !== PROP_TYPES.size ||
        enumerators.some((e) => !PROP_TYPES.has(e))
      ) {
        fail(
          `enum PropType enumerators ${JSON.stringify(enumerators)} do not ` +
            `match the canonical PropType set ${
              JSON.stringify([...PROP_TYPES])
            }`,
        );
      }
      this.openfxInner.push("enum:propType");
    });
  });

  // `enum class PropId { ... };` -- only the member names matter (they are
  // the ids referenced by DEFINE_PROP_TRAITS).
  enumPropId = this.RULE("enumPropId", () => {
    this.CONSUME(EnumKW);
    this.CONSUME(ClassKW);
    const name = this.CONSUME(Identifier);
    this.ACTION(() => {
      if (name.image !== "PropId") {
        fail(`expected enum name "PropId" but got "${name.image}"`);
      }
    });
    this.CONSUME(LBrace);
    const first = this.CONSUME2(Identifier);
    this.ACTION(() => this.propIdNames.push(first.image));
    this.MANY(() => {
      this.CONSUME(Comma);
      const next = this.CONSUME3(Identifier);
      this.ACTION(() => this.propIdNames.push(next.image));
    });
    this.CONSUME(RBrace);
    this.CONSUME(Semicolon);
    this.ACTION(() => {
      const sentinel = this.propIdNames.pop();
      if (sentinel !== "NProps") {
        fail(
          `expected the PropId enum to end with the "NProps" sentinel but got "${
            sentinel ?? "<none>"
          }"`,
        );
      }
      if (new Set(this.propIdNames).size !== this.propIdNames.length) {
        fail("duplicate members in the PropId enum");
      }
    });
    this.ACTION(() => this.openfxInner.push("enum:propId"));
  });

  // namespace prop_enum_values { constexpr std::array X = {...}; ... }
  namespacePropEnumValues = this.RULE("namespacePropEnumValues", () => {
    this.CONSUME(NamespaceKW);
    this.CONSUME(PropEnumValuesNS);
    this.CONSUME(LBrace);
    this.MANY(() => this.SUBRULE(this.enumValuesDecl));
    this.CONSUME(RBrace);
    this.ACTION(() => this.openfxInner.push("namespace:propEnumValues"));
  });

  enumValuesDecl = this.RULE("enumValuesDecl", () => {
    this.CONSUME(ConstexprKW);
    this.CONSUME(StdArray);
    const nameTok = this.CONSUME(Identifier);
    this.CONSUME(Equals);
    this.CONSUME(LBrace);
    const values: string[] = [];
    const first = this.CONSUME(StringLiteral);
    this.ACTION(() => values.push(parseCppString(first.image)));
    this.MANY(() => {
      this.CONSUME(Comma);
      const next = this.CONSUME2(StringLiteral);
      this.ACTION(() => values.push(parseCppString(next.image)));
    });
    this.CONSUME(RBrace);
    this.CONSUME(Semicolon);
    this.ACTION(() => {
      const name = nameTok.image;
      if (name in this.propEnumValues) {
        fail(`duplicate prop_enum_values entry "${name}"`);
      }
      this.propEnumValues[name] = new Set(values);
    });
  });

  // namespace prop_type_arrays { static constexpr PropType X_types[] = {...}; ... }
  namespacePropTypeArrays = this.RULE("namespacePropTypeArrays", () => {
    this.CONSUME(NamespaceKW);
    this.CONSUME(PropTypeArraysNS);
    this.CONSUME(LBrace);
    this.MANY(() => this.SUBRULE(this.typeArrayDecl));
    this.CONSUME(RBrace);
    this.ACTION(() => this.openfxInner.push("namespace:propTypeArrays"));
  });

  typeArrayDecl = this.RULE("typeArrayDecl", () => {
    this.CONSUME(StaticKW);
    this.CONSUME(ConstexprKW);
    this.CONSUME(PropTypeKW);
    const nameTok = this.CONSUME(Identifier);
    this.CONSUME(LBracket);
    this.CONSUME(RBracket);
    this.CONSUME(Equals);
    this.CONSUME(LBrace);
    const types: string[] = [];
    this.CONSUME2(PropTypeKW);
    this.CONSUME(ScopeRes);
    const first = this.CONSUME2(Identifier);
    this.ACTION(() => types.push(first.image));
    this.MANY(() => {
      this.CONSUME(Comma);
      this.CONSUME3(PropTypeKW);
      this.CONSUME2(ScopeRes);
      const next = this.CONSUME3(Identifier);
      this.ACTION(() => types.push(next.image));
    });
    this.CONSUME(RBrace);
    this.CONSUME(Semicolon);
    this.ACTION(() => {
      const name = nameTok.image;
      if (!name.endsWith("_types")) {
        fail(`prop_type_arrays entry "${name}" lacks the "_types" suffix`);
      }
      if (name in this.propTypeArrays) {
        fail(`duplicate prop_type_arrays entry "${name}"`);
      }
      this.propTypeArrays[name] = new Set(types as PropType[]);
    });
  });

  // struct PropDef { ... }; -- body is irrelevant to the result.
  structPropDef = this.RULE("structPropDef", () => {
    this.CONSUME(StructKW);
    const name = this.CONSUME(Identifier);
    this.ACTION(() => {
      if (name.image !== "PropDef") {
        fail(`expected struct name "PropDef" but got "${name.image}"`);
      }
    });
    this.SUBRULE(this.braceBlock);
    this.CONSUME(Semicolon);
    this.ACTION(() => this.openfxInner.push("struct:propDef"));
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

  // template <...> struct PropDefsArray { ... }; -- parameters and body are
  // irrelevant to the result.
  templateStructPropDefsArray = this.RULE("templateStructPropDefsArray", () => {
    this.CONSUME(TemplateKW);
    this.SUBRULE(this.templateParams);
    this.CONSUME(StructKW);
    const name = this.CONSUME(Identifier);
    this.ACTION(() => {
      if (name.image !== "PropDefsArray") {
        fail(`expected struct name "PropDefsArray" but got "${name.image}"`);
      }
    });
    this.SUBRULE(this.braceBlock);
    this.CONSUME(Semicolon);
    this.ACTION(() => this.openfxInner.push("struct:propDefsArray"));
  });

  templateParams = this.RULE("templateParams", () => {
    this.CONSUME(Less);
    this.SUBRULE(this.templateParam);
    this.MANY(() => {
      this.CONSUME(Comma);
      this.SUBRULE2(this.templateParam);
    });
    this.CONSUME(Greater);
  });

  templateParam = this.RULE("templateParam", () => {
    this.AT_LEAST_ONE(() => this.CONSUME(AnyExceptAngleOrComma));
  });

  // static inline constexpr PropDefsArray<PropDef> prop_defs = {{{ ... }}};
  staticPropDefs = this.RULE("staticPropDefs", () => {
    this.CONSUME(StaticKW);
    this.CONSUME(InlineKW);
    this.CONSUME(ConstexprKW);
    const arrayType = this.CONSUME(Identifier);
    this.CONSUME(Less);
    const elemType = this.CONSUME2(Identifier);
    this.CONSUME(Greater);
    this.CONSUME(PropDefsVar);
    this.ACTION(() => {
      if (arrayType.image !== "PropDefsArray") {
        fail(`expected "PropDefsArray" but got "${arrayType.image}"`);
      }
      if (elemType.image !== "PropDef") {
        fail(`expected "PropDef" but got "${elemType.image}"`);
      }
    });
    this.CONSUME(Equals);
    this.CONSUME(LBrace);
    this.CONSUME2(LBrace);
    this.CONSUME3(LBrace);
    // Every entry (including the last) is followed by a comma.
    this.AT_LEAST_ONE(() => {
      this.SUBRULE(this.propDefEntry);
      this.CONSUME(Comma);
    });
    this.CONSUME(RBrace);
    this.CONSUME2(RBrace);
    this.CONSUME3(RBrace);
    this.CONSUME(Semicolon);
    this.ACTION(() => this.openfxInner.push("static:propDefs"));
  });

  // { "PropName", openfx::span(prop_type_arrays::PropName_types, N), DIM, <enum values span> }
  propDefEntry = this.RULE("propDefEntry", () => {
    this.CONSUME(LBrace);
    const nameTok = this.CONSUME(StringLiteral);
    this.CONSUME(Comma);
    // openfx::span(prop_type_arrays::X_types, N)
    this.CONSUME(OpenfxSpan);
    this.CONSUME(LParen);
    this.CONSUME(PropTypeArraysNS);
    this.CONSUME(ScopeRes);
    const typesNameTok = this.CONSUME(Identifier);
    this.CONSUME2(Comma);
    const countTok = this.CONSUME(NumberLiteral);
    this.CONSUME(RParen);
    this.CONSUME3(Comma);
    const dimensionTok = this.CONSUME2(NumberLiteral);
    this.CONSUME4(Comma);
    this.SUBRULE(this.enumValuesSpan);
    this.CONSUME(RBrace);
    this.ACTION(() => {
      const name = parseCppString(nameTok.image);
      const typesName = typesNameTok.image;
      if (typesName !== `${name}_types`) {
        fail(
          `prop_defs entry "${name}" references unexpected type array "${typesName}"`,
        );
      }
      const types = this.propTypeArrays[typesName];
      if (types === undefined) {
        fail(
          `prop_defs entry "${name}" references unknown type array "${typesName}"`,
        );
      }
      const count = toInt(countTok.image, "type count");
      if (types.size !== count) {
        fail(
          `prop_defs entry "${name}": span count ${count} does not match ${types.size} type(s) in "${typesName}"`,
        );
      }
      const dimension = toInt(dimensionTok.image, "dimension");
      if (name in this.propDefsArray) {
        fail(`duplicate prop_defs entry "${name}"`);
      }
      (this.propDefsArray[name] ??= []).push({ dimension });
    });
  });

  enumValuesSpan = this.RULE("enumValuesSpan", () => {
    this.CONSUME(OpenfxSpan);
    this.OR([
      { ALT: () => this.SUBRULE(this.emptyEnumValuesSpan) },
      { ALT: () => this.SUBRULE(this.referencedEnumValuesSpan) },
    ]);
  });

  // openfx::span<const char* const>()
  emptyEnumValuesSpan = this.RULE("emptyEnumValuesSpan", () => {
    this.CONSUME(Less);
    this.CONSUME(ConstKW);
    const elemType = this.CONSUME(Identifier);
    this.CONSUME(Star);
    this.CONSUME2(ConstKW);
    this.CONSUME(Greater);
    this.CONSUME(LParen);
    this.CONSUME(RParen);
    this.ACTION(() => {
      if (elemType.image !== "char") {
        fail(`unexpected span element type "${elemType.image}"`);
      }
    });
  });

  // openfx::span(prop_enum_values::X.data(), prop_enum_values::X.size())
  referencedEnumValuesSpan = this.RULE("referencedEnumValuesSpan", () => {
    this.CONSUME(LParen);
    this.CONSUME(PropEnumValuesNS);
    this.CONSUME(ScopeRes);
    const dataName = this.CONSUME(Identifier);
    this.CONSUME(Dot);
    this.CONSUME2(Identifier);
    this.CONSUME2(LParen);
    this.CONSUME(RParen);
    this.CONSUME(Comma);
    this.CONSUME2(PropEnumValuesNS);
    this.CONSUME2(ScopeRes);
    const sizeName = this.CONSUME3(Identifier);
    this.CONSUME2(Dot);
    this.CONSUME4(Identifier);
    this.CONSUME3(LParen);
    this.CONSUME2(RParen);
    this.CONSUME3(RParen);
    this.ACTION(() => {
      if (dataName.image !== sizeName.image) {
        fail(
          `enum values span references "${dataName.image}" and "${sizeName.image}"`,
        );
      }
      if (!(dataName.image in this.propEnumValues)) {
        fail(
          `enum values span references unknown prop_enum_values entry "${dataName.image}"`,
        );
      }
    });
  });

  // namespace properties { PropTraits template; #define ...; calls; }
  namespaceProperties = this.RULE("namespaceProperties", () => {
    this.CONSUME(NamespaceKW);
    this.CONSUME(PropertiesNS);
    this.CONSUME(LBrace);
    this.SUBRULE(this.propTraitsTemplateDecl);
    this.OPTION(() => this.CONSUME(DefineDirective));
    this.AT_LEAST_ONE(() => this.SUBRULE(this.definePropTraitsCall));
    this.CONSUME(RBrace);
    this.ACTION(() => {
      // Cross-check: every DEFINE_PROP_TRAITS id must be a distinct PropId
      // member, and every PropId member must have exactly one call.
      const seen = new Set<string>();
      for (const id of this.propTraitsCallIds) {
        if (seen.has(id)) {
          fail(`duplicate DEFINE_PROP_TRAITS entry "${id}"`);
        }
        seen.add(id);
        if (!this.propIdNames.includes(id)) {
          fail(
            `DEFINE_PROP_TRAITS references "${id}", which is not a PropId member`,
          );
        }
      }
      if (seen.size !== this.propIdNames.length) {
        fail(
          `${this.propIdNames.length} PropId members but ${seen.size} ` +
            `DEFINE_PROP_TRAITS calls`,
        );
      }
      this.propertiesInner.push("definePropTraits[]");
      this.openfxInner.push({
        type: "namespace:properties",
        inner: [...this.propertiesInner] as PropertiesInner,
      });
    });
  });

  // template<PropId id> struct PropTraits;
  propTraitsTemplateDecl = this.RULE("propTraitsTemplateDecl", () => {
    this.CONSUME(TemplateKW);
    this.CONSUME(Less);
    const idType = this.CONSUME(Identifier);
    this.CONSUME2(Identifier);
    this.CONSUME(Greater);
    this.CONSUME(StructKW);
    const name = this.CONSUME3(Identifier);
    this.CONSUME(Semicolon);
    this.ACTION(() => {
      if (idType.image !== "PropId") {
        fail(
          `expected template parameter type "PropId" but got "${idType.image}"`,
        );
      }
      if (name.image !== "PropTraits") {
        fail(`expected struct name "PropTraits" but got "${name.image}"`);
      }
      this.propertiesInner.push("struct:propTraits");
    });
  });

  // DEFINE_PROP_TRAITS(id, type, isMultitype);
  definePropTraitsCall = this.RULE("definePropTraitsCall", () => {
    this.CONSUME(DefinePropTraitsKW);
    this.CONSUME(LParen);
    const id = this.CONSUME(Identifier);
    this.CONSUME(Comma);
    this.SUBRULE(this.cppType);
    this.CONSUME2(Comma);
    this.CONSUME2(Identifier);
    this.CONSUME(RParen);
    this.CONSUME(Semicolon);
    this.ACTION(() => this.propTraitsCallIds.push(id.image));
  });

  // const? char* | bool | void* | double | int
  cppType = this.RULE("cppType", () => {
    this.OPTION(() => this.CONSUME(ConstKW));
    this.CONSUME(Identifier);
    this.OPTION2(() => this.CONSUME(Star));
  });

  // namespace assertions { using-decls; static_asserts; }
  namespaceAssertions = this.RULE("namespaceAssertions", () => {
    this.CONSUME(NamespaceKW);
    this.CONSUME(AssertionsNS);
    this.CONSUME(LBrace);
    this.MANY(() => this.SUBRULE(this.usingDeclaration));
    this.ACTION(() => this.assertionsInner.push("using[]"));
    this.AT_LEAST_ONE(() => this.SUBRULE(this.staticAssertDeclaration));
    this.CONSUME(RBrace);
    this.ACTION(() => {
      this.assertionsInner.push("staticAssert[]");
      this.openfxInner.push({
        type: "namespace:assertions",
        inner: [...this.assertionsInner] as AssertionsInner,
      });
    });
  });

  usingDeclaration = this.RULE("usingDeclaration", () => {
    this.CONSUME(UsingKW);
    this.CONSUME(Identifier);
    this.MANY(() => {
      this.CONSUME(ScopeRes);
      this.CONSUME2(Identifier);
    });
    this.CONSUME(Semicolon);
  });

  staticAssertDeclaration = this.RULE("staticAssertDeclaration", () => {
    this.CONSUME(StaticAssertKW);
    this.CONSUME(LParen);
    this.SUBRULE(this.assertExpression);
    this.CONSUME(RParen);
    this.CONSUME(Semicolon);
  });

  // The assert expression itself is not recorded; only its parenthesized
  // shape is enforced.
  assertExpression = this.RULE("assertExpression", () => {
    this.AT_LEAST_ONE(() => {
      this.OR([
        { ALT: () => this.CONSUME(AnyExceptParenOrSemicolon) },
        { ALT: () => this.SUBRULE(this.parenthesizedExpression) },
      ]);
    });
  });

  parenthesizedExpression = this.RULE("parenthesizedExpression", () => {
    this.CONSUME(LParen);
    this.MANY(() => this.SUBRULE(this.assertExpression));
    this.CONSUME(RParen);
  });
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

const HEADER_NAME = "ofxPropsMetadata.h";

export function parse(headerCode: string): Result {
  const lexingResult = ofxLexer.tokenize(headerCode);
  if (lexingResult.errors.length > 0) {
    lexingError(HEADER_NAME, headerCode, lexingResult.errors as LexIssue[]);
  }

  const parser = new OfxPropsMetadataParser();
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
        inner: parser.openfxInner as unknown as OpenfxInner,
      },
    ],
    propertyInfos: {
      propEnumValues: parser.propEnumValues,
      propTypeArrays: parser.propTypeArrays,
      propDefsArray: parser.propDefsArray,
    },
  };
}

export function makeFinalResult(result: Result): FinalResult {
  const { propEnumValues, propTypeArrays, propDefsArray } =
    result.propertyInfos;
  const propertyInfos: FinalResult["propertyInfos"] = {};

  for (const [name, entries] of Object.entries(propDefsArray)) {
    const entry = entries[0];
    if (entry === undefined) {
      fail(`no prop_defs entry recorded for "${name}"`);
    }
    const types = propTypeArrays[`${name}_types`];
    if (types === undefined) {
      fail(`missing prop_type_arrays entry for "${name}"`);
    }
    // Set iteration order is insertion order, i.e. header order; the first
    // type is the property's primary type (matching DEFINE_PROP_TRAITS).
    const primary: PropType | undefined = [...types][0];
    if (primary === undefined) {
      fail(`empty prop_type_arrays entry for "${name}"`);
    }
    propertyInfos[name] = {
      type: primary === "Enum"
        ? { Enum: propEnumValues[name] ?? new Set<string>() }
        : primary,
      dimension: entry.dimension,
    };
  }

  return { propertyInfos };
}
