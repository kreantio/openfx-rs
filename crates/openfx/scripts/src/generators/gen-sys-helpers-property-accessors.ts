import path from "node:path";

import {
  FinalResult as FinalResultOfxPropsMetadata,
} from "../vibe-zone/parsers/parser-ofxPropsMetadata/types.ts";
import { NameRegulator } from "../utils/name-regulator.ts";
import { representTypeWithContainer } from "../utils/representations.ts";

export async function genSysHelpersPropertyAccessors(
  fr: FinalResultOfxPropsMetadata,
  opts: {
    nameRegulator: NameRegulator;
    dataIntermediatePath: string;
  },
): Promise<{ image_effect_v1: Record<string, string> }> {
  const partsPerMod = await genAccessors(fr, opts);
  const codePerMod: Record<string, string> = {};
  for (const [mod, parts] of Object.entries(partsPerMod)) {
    codePerMod[mod] = parts.join("\n");
  }

  return { image_effect_v1: codePerMod };
}

/**
 * @returns a record where the keys are the module names and the values are
 * arrays of strings representing the generated code for each module.
 */
async function genAccessors(
  fr: FinalResultOfxPropsMetadata,
  opts: {
    nameRegulator: NameRegulator;
    dataIntermediatePath: string;
  },
): Promise<Record<string, string[]>> {
  const ret: Record<string, string[]> = {};

  const rootItemIdentsPerHeader = JSON.parse(
    await Deno.readTextFile(
      path.join(opts.dataIntermediatePath, "root_item_idents_per_header.json"),
    ),
  );
  for (const k in rootItemIdentsPerHeader) {
    rootItemIdentsPerHeader[k] = new Set(rootItemIdentsPerHeader[k]);
  }

  for (const [keyConstant, v] of Object.entries(fr.propertyInfos)) {
    const name = opts.nameRegulator
      .keyConstantToCanonicalName(keyConstant);
    if (name != keyConstant) {
      console.info(
        `NOTE(gen-sys-helpers-property-accessors): The property with key constant \`${keyConstant}\` has a different canonical name \`${name}\`.`,
      );
    }
    const kName = opts.nameRegulator.keyConstantToKName(keyConstant);

    const mod = findMod(rootItemIdentsPerHeader, kName);
    const parts = (ret[mod] ??= []);

    const possibleTypes = (() => {
      if (v.type instanceof Set) {
        return [...v.type].toSorted();
      } else if (typeof v.type === "object") {
        v.type satisfies { "Enum": unknown };
        return ["String"];
      } else {
        return [v.type];
      }
    })().map((t) => t === "Bool" ? "Int" : t);
    const ty = possibleTypes.length === 1
      ? possibleTypes[0]
      : `(${possibleTypes.join(" | ")})`;
    const tyContainer = representTypeWithContainer(ty, v.dimension);

    const fns = [
      "set",
      "get",
      "reset",
      ...(v.dimension === 0 ? ["get_dimensions"] : []),
    ];

    parts.push(`    ${name}: ${tyContainer} { ${fns.join(" ")} };`);
  }

  for (const mod in ret) {
    ret[mod].splice(
      0,
      0,
      "openfx_internal_macros::sys_helpers_make_property_accessors! {",
    );
    ret[mod].push("}");
  }

  return ret;
}

function findMod(
  rootItemIdentsPerHeader: Record<string, Set<string>>,
  kName: string,
): string {
  for (const [header, idents] of Object.entries(rootItemIdentsPerHeader)) {
    if (idents.has(kName)) return header;
  }
  throw new Error(`Module not found for \`${kName}\``);
}
