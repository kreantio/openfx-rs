import path from "node:path";

import {
  FinalResult as FinalResultOfxPropsMetadata,
  PropType,
} from "../parsers/parser-ofxPropsMetadata/types.ts";
import { NameRegulator } from "../utils/name-regulator.ts";
import { representTypeWithContainer } from "../utils/representations.ts";

export async function genSysHelpersPropertyAccessors(
  fr: FinalResultOfxPropsMetadata,
  opts: {
    nameRegulator: NameRegulator;
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

  parts.push(
    "openfx_internal_macros::sys_helpers_make_property_accessors_by_types! {",
  );

  const typeToPossibleDimensions: Record<PropTypeX, Set<number>> = {
    "Int": new Set(),
    "Double": new Set(),
    "String": new Set(),
    "Pointer": new Set(),
  };
  for (const v of Object.values(fr.propertyInfos)) {
    let vType = v.type;
    if (vType instanceof Set) {
      for (let t of vType) {
        if (t === "Bool") {
          t = "Int";
        }
        typeToPossibleDimensions[t].add(v.dimension);
      }
    } else {
      if (typeof vType !== "string") {
        vType satisfies { "Enum": unknown };
        vType = "String";
      } else if (vType === "Bool") {
        vType = "Int";
      }
      typeToPossibleDimensions[vType].add(v.dimension);
    }
  }

  for (
    const [ty_, ds_] of Object.entries(typeToPossibleDimensions).toSorted()
  ) {
    const ty = ty_ as PropTypeX;
    ds_.add(0);
    ds_.add(1);
    const ds = [...ds_].filter((d) => d != 0 && d != 1).toSorted();
    let part = `    ${ty}: ... pub { set get }, 1 pub { set get }`;
    if (ds.length > 0) {
      part += ", ";
      if (ds.length === 1) {
        part += `${ds[0]}`;
      } else {
        part += `(${ds.join("|")})`;
      }
      part += " pub(crate) { set get }";
    }
    part += ";";
    parts.push(part);
  }

  parts.push("}");
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
