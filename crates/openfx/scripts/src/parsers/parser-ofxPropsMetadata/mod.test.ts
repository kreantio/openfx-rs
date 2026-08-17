import path from "node:path";

import { assert } from "@std/assert";
import { parseArgs } from "@std/cli/parse-args";

import { makeFinalResult, parse } from "./impl-by-llms/mod.ts";
import { FinalResult, PropertyInfos, Structure } from "./types.ts";

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

Deno.test("parsing", async (t) => {
  await t.step("structure", () => {
    Structure.parse(result.structure);
  });

  await t.step("propertyInfos", () => {
    assert(Object.keys(result.propertyInfos.propEnumValues).length > 0);
    assert(Object.keys(result.propertyInfos.propTypeArrays).length > 0);
    assert(Object.keys(result.propertyInfos.propDefsArray).length > 0);
    PropertyInfos.parse(result.propertyInfos);
  });
});

Deno.test("finalResult", (_t) => {
  assert(Object.keys(finalResult.propertyInfos).length > 0);
  FinalResult.parse(finalResult);
});
