// Additional tests for the behaviors introduced by INSTRUCTIONS.md
// "Instructions for update 1": `prop_type_arrays` entries with multiple
// types, and the rule that an enum-typed property must not have any other
// type. This module is imported by ../mod.test.ts and runs under `deno test`
// with the same `--crate-path` argument.

import path from "node:path";

import { assert, assertEquals, assertThrows } from "@std/assert";
import { parseArgs } from "@std/cli/parse-args";

import { makeFinalResult, parse } from "./mod.ts";
import type { PropType, Result } from "../types.ts";

const args = parseArgs(Deno.args, {
  string: ["crate-path"],
});

if (!args["crate-path"]) throw new Error("Missing `--crate-path`");

const headerPath = path.join(
  args["crate-path"],
  "vendor",
  "openfx",
  "openfx-cpp",
  "include",
  "openfx",
  "ofxPropsMetadata.h",
);

const headerCode = await Deno.readTextFile(headerPath);
const result = parse(headerCode);
const finalResult = makeFinalResult(result);

/** The first `*_types` array entry satisfying `pred`, as [name, types]. */
function findTypeArray(
  res: Result,
  pred: (types: ReadonlySet<string>) => boolean,
): [string, Set<PropType>] {
  const entry = Object.entries(res.propertyInfos.propTypeArrays).find(([, t]) =>
    pred(t)
  );
  assert(entry !== undefined, "no matching prop_type_arrays entry in header");
  return entry;
}

Deno.test("update 1: multitype properties keep the full type set", () => {
  // The acceptance example from INSTRUCTIONS.md.
  const type = finalResult.propertyInfos["OfxParamPropDefault"].type;
  assert(type instanceof Set, "expected OfxParamPropDefault to be multitype");
  assertEquals(type, new Set(["Int", "Double", "String", "Pointer"]));
});

Deno.test("update 1: multitype sets mirror the raw type arrays", () => {
  for (
    const [typesName, types] of Object.entries(
      result.propertyInfos.propTypeArrays,
    )
  ) {
    if (types.has("Enum") || types.size === 1) continue;
    const name = typesName.replace(/_types$/, "");
    const info = finalResult.propertyInfos[name];
    assert(info !== undefined, `no final entry for multitype "${name}"`);
    assert(info.type instanceof Set, `expected "${name}" to stay a Set`);
    assertEquals(info.type, types);
  }
});

Deno.test("single-type properties collapse to the bare type", () => {
  for (
    const [typesName, types] of Object.entries(
      result.propertyInfos.propTypeArrays,
    )
  ) {
    const [only] = types;
    if (types.size !== 1 || only === "Enum") continue;
    const name = typesName.replace(/_types$/, "");
    const info = finalResult.propertyInfos[name];
    assert(info !== undefined, `no final entry for "${name}"`);
    assertEquals(info.type, only);
  }
});

Deno.test("enum-typed properties map to { Enum: name } with values", () => {
  for (
    const [typesName, types] of Object.entries(
      result.propertyInfos.propTypeArrays,
    )
  ) {
    if (!types.has("Enum")) continue;
    const name = typesName.replace(/_types$/, "");
    const info = finalResult.propertyInfos[name];
    assert(info !== undefined, `no final entry for enum-typed "${name}"`);
    assertEquals(info.type, { Enum: name });
    assert(
      name in finalResult.propEnumValues,
      `enum-typed "${name}" has no prop_enum_values entry`,
    );
  }
});

Deno.test("update 1: mixing Enum with another type throws", () => {
  // "Int" added to an enum-only array.
  const enumAdded = structuredClone(result);
  findTypeArray(enumAdded, (t) => t.has("Enum"))[1].add("Int");
  assertThrows(() => makeFinalResult(enumAdded), Error, `mixes "Enum"`);

  // "Enum" added to a multitype (non-enum) array.
  const multiAdded = structuredClone(result);
  findTypeArray(multiAdded, (t) => t.size > 1 && !t.has("Enum"))[1].add(
    "Enum",
  );
  assertThrows(() => makeFinalResult(multiAdded), Error, `mixes "Enum"`);

  // The check fires even for arrays not referenced by any prop_defs entry.
  const unreferenced = structuredClone(result);
  const [typesName, types] = findTypeArray(unreferenced, (t) => t.has("Enum"));
  delete unreferenced.propertyInfos.propDefsArray[
    typesName.replace(/_types$/, "")
  ];
  types.add("Int");
  assertThrows(() => makeFinalResult(unreferenced), Error, `mixes "Enum"`);
});

Deno.test("update 1: unknown PropType enumerator is rejected", () => {
  // Rewrite the first single-entry type array to use a bogus enumerator.
  const mutated = headerCode.replace(
    /\{PropType::[A-Za-z]+\}/,
    "{PropType::Bogus}",
  );
  assert(mutated !== headerCode, "no single-entry type array found to mutate");
  assertThrows(() => parse(mutated), Error, "unknown type");
});
