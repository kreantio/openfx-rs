import path from "node:path";

import { parseArgs } from "@std/cli/parse-args";

import * as toml from "toml";

import {
  makeFinalResult as makeFinalResultOfxPropsMetadata,
  parse as parseOfxPropsMetadata,
} from "../src/parsers/parser-ofxPropsMetadata/impl-by-llms/mod.ts";

import { CodegenConfig } from "../src/definitions.ts";
import { genLowEnums } from "../src/generators/gen-low-enums.ts";
import { genSysHelpersPropertyAccessors } from "../src/generators/gen-sys-helpers-property-accessors.ts";

function doParseArgs(args: string[]) {
  const result = parseArgs(args, {
    string: [
      "codegen-config",
      "input-cpp-headers",
      "input-data-from-c",
      "output-code-from-cpp",
    ],
  });

  if (!result["codegen-config"]) {
    throw new Error("Missing `--codegen-config`");
  }
  if (!result["input-cpp-headers"]) {
    throw new Error("Missing `--input-cpp-headers`");
  }
  if (!result["output-code-from-cpp"]) {
    throw new Error("Missing `--output-code-from-cpp`");
  }
  if (!result["input-data-from-c"]) {
    throw new Error("Missing `--input-data-from-c`");
  }

  return result;
}
type _Args = ReturnType<typeof doParseArgs>;
type Args = Required<_Args>;

function parseCodegenConfig(tomlText: string): CodegenConfig {
  return toml.parse(tomlText);
}

async function main(args: Args) {
  const codegenConfig = parseCodegenConfig(
    await Deno.readTextFile(args["codegen-config"]),
  );

  const propsMetadata = makeFinalResultOfxPropsMetadata(parseOfxPropsMetadata(
    await Deno.readTextFile(
      path.join(args["input-cpp-headers"], "ofxPropsMetadata.h"),
    ),
  ));

  await Deno.writeTextFile(
    path.join(args["output-code-from-cpp"], "low_enums.rs"),
    genLowEnums(propsMetadata, codegenConfig),
  );
  {
    const { generic, image_effect_v1: codePerMod } =
      await genSysHelpersPropertyAccessors(
        propsMetadata,
        codegenConfig,
        { dataFromCPath: args["input-data-from-c"] },
      );
    await Deno.writeTextFile(
      path.join(
        args["output-code-from-cpp"],
        "sys_helpers_property_accessors_generic.rs",
      ),
      generic,
    );
    await Deno.mkdir(
      path.join(args["output-code-from-cpp"], "sys_helpers_property_accessors"),
      { recursive: true },
    );
    for (const [mod, code] of Object.entries(codePerMod)) {
      await Deno.writeTextFile(
        path.join(
          args["output-code-from-cpp"],
          "sys_helpers_property_accessors",
          `${mod}.rs`,
        ),
        code,
      );
    }
  }
}

await main(doParseArgs(Deno.args) as Args);
