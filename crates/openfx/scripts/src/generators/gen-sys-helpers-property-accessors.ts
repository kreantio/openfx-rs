import path from "node:path";

import { CodegenConfig } from "../definitions.ts";
import {
  FinalResult as FinalResultOfxPropsMetadata,
  PropType,
} from "../parsers/parser-ofxPropsMetadata/types.ts";

export async function genSysHelpersPropertyAccessors(
  fr: FinalResultOfxPropsMetadata,
  cfg: CodegenConfig,
  opts: { dataFromCPath: string },
): Promise<{ generic: string; image_effect_v1: Record<string, string> }> {
  const partsGeneric: string[] = [];

  genAccessorsForTypesWithDimensions(partsGeneric, fr);

  const partsPerMod = await genAccessors(fr, cfg, opts);
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
    if (typeof v_type !== "string") {
      v_type satisfies { "Enum": unknown };
      v_type = "String";
    } else if (v_type === "Bool") {
      v_type = "Int";
    }
    typeToPossibleDimensions[v_type].add(v.dimension);
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
  cfg: CodegenConfig,
  opts: { dataFromCPath: string },
): Promise<Record<string, string[]>> {
  const ret: Record<string, string[]> = {};

  const rootItemIdentsPerHeader = JSON.parse(
    await Deno.readTextFile(
      path.join(opts.dataFromCPath, "root_item_idents_per_header.json"),
    ),
  );
  for (const k in rootItemIdentsPerHeader) {
    rootItemIdentsPerHeader[k] = new Set(rootItemIdentsPerHeader[k]);
  }

  for (let [k, v] of Object.entries(fr.propertyInfos)) {
    const fix = cfg.property_value_to_key_exceptions[k];
    if (fix) {
      console.info(
        `Fix: replacing property name "${k}" with "${fix}"`,
      );
      k = fix;
    }

    let v_type = v.type;
    if (typeof v_type !== "string") {
      v_type satisfies { "Enum": unknown };
      v_type = "String";
    } else if (v_type === "Bool") {
      v_type = "Int";
    }

    const kName = `k${k}`;
    const mod = findMod(rootItemIdentsPerHeader, kName);
    const parts = (ret[mod] ??= []);

    const fnNameS = getFnName("set", v_type, v.dimension, true);
    const fnNameG = getFnName("get", v_type, v.dimension, true);
    if (v.dimension === 0) {
      parts.push(...[
        `make_property_setter!(set_${k}, ${kName}, ${fnNameS}, ..., ${v_type});`,
        `make_property_getter!(get_${k}, ${kName}, ${fnNameG}, ..., ${v_type});`,
      ]);
    } else if (v.dimension === 1) {
      parts.push(...[
        `make_property_setter!(set_${k}, ${kName}, ${fnNameS}, ${v.dimension}, ${v_type});`,
        `make_property_getter!(get_${k}, ${kName}, ${fnNameG}, ${v.dimension}, ${v_type});`,
      ]);
    } else {
      parts.push(...[
        `make_property_setter!(set_${k}, ${kName}, ${fnNameS}, ${v.dimension}, ${v_type});`,
        `make_property_getter!(get_${k}, ${kName}, ${fnNameG}, ${v.dimension}, ${v_type});`,
      ]);
    }
    parts.push(`make_property_resetter!(reset_${k}, ${kName});`);
    if (v.dimension === 0) {
      parts.push(
        `make_property_dimension_getter!(get_dimension_${k}, ${kName});`,
      );
    }
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
