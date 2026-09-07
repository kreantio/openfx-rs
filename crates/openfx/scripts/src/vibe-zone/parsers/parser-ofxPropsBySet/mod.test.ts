import "./impl-by-llms/additional-tests.ts";

import path from "node:path";

import { assert } from "@std/assert";
import { parseArgs } from "@std/cli/parse-args";

import { makeFinalResult, parse } from "./impl-by-llms/mod.ts";
import { FinalResult, Infos, Structure } from "./types.ts";

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
  "ofxPropsBySet.h",
);

const headerCode = await Deno.readTextFile(headerPath);
const result = parse(headerCode);
const finalResult = makeFinalResult(result);

Deno.test("parsing", async (t) => {
  await t.step("structure", () => {
    Structure.parse(result.structure);
  });

  await t.step("propertyInfos", () => {
    assert(Object.keys(result.infos.propSets).length > 0);
    assert(result.infos.actions.size > 0);
    assert(Object.keys(result.infos.actionProps).length > 0);
    Infos.parse(result.infos);
  });
});

Deno.test("finalResult", (_t) => {
  FinalResult.parse(finalResult);
});
