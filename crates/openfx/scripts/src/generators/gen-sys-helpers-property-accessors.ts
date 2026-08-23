import path from "node:path";

import {
  FinalResult as FinalResultOfxPropsMetadata,
  PropType,
} from "../parsers/parser-ofxPropsMetadata/types.ts";
import { NameRegulator } from "../utils/name-regulator.ts";

export async function genSysHelpersPropertyAccessors(
  fr: FinalResultOfxPropsMetadata,
  opts: {
    propertyNameRegulator: NameRegulator;
    dataIntermediatePath: string;
  },
): Promise<{ generic: string; image_effect_v1: Record<string, string> }> {
  const partsGeneric: string[] = [];

  genAccessorsForTypesWithDimensions(partsGeneric, fr);

  const partsPerMod = await genAccessors(fr, opts);
  const codePerMod: Record<string, string> = {};
  for (const [mod, parts] of Object.entries(partsPerMod)) {
    codePerMod[mod] = parts.join("\n");
  }

  return { generic: partsGeneric.join("\n"), image_effect_v1: codePerMod };
}

function genAccessorsForTypesWithDimensions(
  parts: string[],
  fr: FinalResultOfxPropsMetadata,
): void {
  type PropTypeX = Exclude<PropType, "Enum" | "Bool">;

  const typeToPossibleDimensions: Record<PropTypeX, Set<number>> = {
    "Int": new Set(),
    "Double": new Set(),
    "String": new Set(),
    "Pointer": new Set(),
  };
  for (const v of Object.values(fr.propertyInfos)) {
    let v_type = v.type;
    if (v_type instanceof Set) {
      for (let t of v_type) {
        if (t === "Bool") {
          t = "Int";
        }
        typeToPossibleDimensions[t].add(v.dimension);
      }
    } else {
      if (typeof v_type !== "string") {
        v_type satisfies { "Enum": unknown };
        v_type = "String";
      } else if (v_type === "Bool") {
        v_type = "Int";
      }
      typeToPossibleDimensions[v_type].add(v.dimension);
    }
  }

  for (
    const [ty_, ds_] of Object.entries(typeToPossibleDimensions)
      .toSorted((a, b) => a[0].localeCompare(b[0]))
  ) {
    const ty = ty_ as PropTypeX;
    ds_.add(0);
    ds_.add(1);
    const ds = [...ds_].toSorted();

    for (const d of ds) {
      const fnNameS = getFnName("set", ty, d, false);
      const fnNameG = getFnName("get", ty, d, false);
      const vis = d > 1 ? "pub(crate)" : "pub";
      if (d === 0) {
        parts.push(...[
          `make_property_setter_for_type!(pub ${fnNameS}, ..., ${ty});`,
          `make_property_getter_for_type!(pub ${fnNameG}, ..., ${ty});`,
        ]);
      } else {
        parts.push(...[
          `make_property_setter_for_type!(${vis} ${fnNameS}, ${d}, ${ty});`,
          `make_property_getter_for_type!(${vis} ${fnNameG}, ${d}, ${ty});`,
        ]);
      }
    }
  }
}

/**
 * @returns a record where the keys are the module names and the values are
 * arrays of strings representing the generated code for each module.
 */
async function genAccessors(
  fr: FinalResultOfxPropsMetadata,
  opts: {
    propertyNameRegulator: NameRegulator;
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
    const name = opts.propertyNameRegulator
      .keyConstantToCanonicalName(keyConstant);
    if (name != keyConstant) {
      console.info(
        `NOTE(gen-sys-helpers-property-accessors): The property with key constant \`${keyConstant}\` has a different canonical name \`${name}\`.`,
      );
    }
    const kName = opts.propertyNameRegulator.keyConstantToKName(keyConstant);

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
    const tyContainer = (() => {
      if (v.dimension === 0) {
        return `[${ty}]`;
      } else if (v.dimension === 1) {
        return ty;
      } else {
        return `[${ty}; ${v.dimension}]`;
      }
    })();

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

function getFnName(
  getOrSet: "get" | "set",
  ty: string,
  d: number,
  withPath: boolean,
): string {
  const path = withPath ? "crate::generic::sys_helpers::properties::" : "";

  const tyLower = ty.toLowerCase();
  if (d === 0) {
    return `${path}${getOrSet}_${tyLower}s`;
  } else if (d === 1) {
    return `${path}${getOrSet}_${tyLower}`;
  } else {
    return `${path}${getOrSet}_${tyLower}s_${d}`;
  }
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
