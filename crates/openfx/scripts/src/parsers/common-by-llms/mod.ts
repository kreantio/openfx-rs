// Scaffolding shared between the ofx header parsers
// (`parser-ofxPropMetadata` and `parser-ofxPropsBySet`), moved here per the
// instructions in each parser's `impl-by-llms/INSTRUCTIONS.md`.
//
// Everything here is dialect-agnostic: trivia tokens that carry no token
// categories, preprocessor tokens common to the headers, and small helpers
// used by both implementations. Anything that varies between header dialects
// (soup categories, keywords, error messages) stays in the parsers.

import { createToken, Lexer } from "chevrotain";
import type { TokenType } from "chevrotain";

// ---------------------------------------------------------------------------
// Tokens
// ---------------------------------------------------------------------------

// --- skipped by the lexer ---------------------------------------------------

export const WhiteSpace = createToken({
  name: "WhiteSpace",
  pattern: /\s+/,
  group: Lexer.SKIPPED,
});
export const LineComment = createToken({
  name: "LineComment",
  pattern: /\/\/[^\n\r]*/,
  group: Lexer.SKIPPED,
});
export const BlockComment = createToken({
  name: "BlockComment",
  pattern: /\/\*[\s\S]*?\*\//,
  group: Lexer.SKIPPED,
});

// --- preprocessor -----------------------------------------------------------

// `#pragma once` (and nothing else -- any other pragma fails to lex).
export const PragmaOnce = createToken({
  name: "PragmaOnce",
  pattern: /#pragma[ \t]+once\b/,
});
export const Include = createToken({ name: "Include", pattern: Lexer.NA });
export const IncludeAngle = createToken({
  name: "IncludeAngle",
  pattern: /#include[ \t]*<[^>\n\r]*>/,
  categories: [Include],
});
export const IncludeQuoted = createToken({
  name: "IncludeQuoted",
  pattern: /#include[ \t]*"[^"\n\r]*"/,
  categories: [Include],
});

/**
 * Trivia and preprocessor tokens shared by every header dialect, in lexer
 * order (they must precede any dialect token that could match their text).
 */
export const headerBaseTokens: TokenType[] = [
  WhiteSpace,
  LineComment,
  BlockComment,
  PragmaOnce,
  IncludeAngle,
  IncludeQuoted,
];

/**
 * A lexer for one header dialect: the shared trivia/preprocessor tokens
 * first, then the dialect's own tokens.
 */
export function makeHeaderLexer(dialectTokens: TokenType[]): Lexer {
  return new Lexer([...headerBaseTokens, ...dialectTokens], {
    positionTracking: "onlyOffset",
  });
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/** Throws the given message; used to fail from inside parser ACTIONs. */
export function fail(message: string): never {
  throw new Error(message);
}

/** Maps a token offset in the source to a 1-based line number. */
export function lineOf(source: string, offset: number): number {
  let line = 1;
  for (let i = 0; i < offset && i < source.length; i++) {
    if (source.charCodeAt(i) === 10) line++;
  }
  return line;
}

/**
 * Parses the simple string literals used by the headers. They contain plain
 * quoted identifiers only; anything fancier is a structural change and is
 * rejected.
 */
export function parseCppString(image: string): string {
  if (
    image.length < 2 || image[0] !== '"' || image[image.length - 1] !== '"' ||
    image.slice(1, -1).includes('"') || image.slice(1, -1).includes("\\")
  ) {
    fail(`unsupported string literal: ${image}`);
  }
  return image.slice(1, -1);
}

/** Parses a non-negative decimal integer token image. */
export function toInt(image: string, what: string): number {
  const value = Number(image);
  if (!Number.isSafeInteger(value) || value < 0) {
    fail(`invalid ${what}: ${image}`);
  }
  return value;
}

// ---------------------------------------------------------------------------
// Error formatting
// ---------------------------------------------------------------------------

/** Minimal structural types satisfied by chevrotain's error objects. */
export interface LexIssue {
  message: string;
  offset?: number;
}
export interface ParseIssue {
  message: string;
  token?: { startOffset?: number };
}

/** Collects lexer errors into a single thrown Error (never returns). */
export function lexingError(
  headerName: string,
  source: string,
  errors: LexIssue[],
): never {
  const details = errors.map((e) => {
    const where = e.offset !== undefined
      ? ` (line ${lineOf(source, e.offset)})`
      : "";
    return `${e.message}${where}`;
  }).join("\n");
  throw new Error(`Failed to lex ${headerName}:\n${details}`);
}

/** Collects parser errors into a single thrown Error (never returns). */
export function parsingError(
  headerName: string,
  source: string,
  errors: ParseIssue[],
): never {
  const details = errors.map((e) => {
    const where = e.token?.startOffset !== undefined
      ? ` (line ${lineOf(source, e.token.startOffset)})`
      : "";
    return `${e.message}${where}`;
  }).join("\n");
  const error = new Error(`Failed to parse ${headerName}:\n${details}`);
  // Attach the first offending token for programmatic inspection.
  Object.assign(error, { token: errors[0]?.token });
  throw error;
}
