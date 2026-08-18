import path from "node:path";

import { parseArgs } from "@std/cli/parse-args";

import * as toml from "toml";

import {
  makeFinalResult as makeFinalResultOfxPropsMetadata,
  parse as parseOfxPropsMetadata,
} from "../src/parsers/parser-ofxPropsMetadata/impl-by-llms/mod.ts";
import {
  FinalResult as FinalResultOfxPropsMetadata,
} from "../src/parsers/parser-ofxPropsMetadata/types.ts";

function doParseArgs(args: string[]) {
  const result = parseArgs(args, {
    string: [
      "codegen-config",
      "input-cpp-headers",
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

  return result;
}
type _Args = ReturnType<typeof doParseArgs>;
type Args = Required<_Args>;

interface CodegenConfig {
  "property_value_to_key_exceptions": Record<string, string>;
}

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
}

function genLowEnums(
  fr: FinalResultOfxPropsMetadata,
  cfg: CodegenConfig,
): string {
  const items: string[] = [];
  for (
    const [name, values] of Object.entries(fr.propEnumValues)
      .toSorted((a, b) => a[0].localeCompare(b[0]))
  ) {
    const valuesList = [...values]
      .map((v) => {
        const fix = cfg.property_value_to_key_exceptions[v];
        if (fix) {
          console.info(
            `Fix: replacing enum variant name "${v}" with "${fix}" for property \`${name}\``,
          );
          return fix;
        }
        return v;
      })
      .toSorted();

    if (valuesList[0]!.startsWith("Ofx")) {
      items.push(
        `crate::internal::low_macros::make_enum_from_paths!(${name},${
          valuesList.map((v) => {
            const path = `crate::sys_umbrella::k${v}`;
            return `\n    /// See: [\`${path}\`].\n    ${v} => ${path}`;
          }).join(", ")
        }\n);`,
      );
    } else {
      items.push(
        `crate::internal::low_macros::make_enum_from_idents!(${name},${
          valuesList.map((v) => `\n    r#${v} : c"${v}"`).join(", ")
        }\n);`,
      );
    }
  }

  return items.join("\n");
}

await main(doParseArgs(Deno.args) as Args);
