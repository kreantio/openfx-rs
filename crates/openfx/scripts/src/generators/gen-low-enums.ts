import {
  FinalResult as FinalResultOfxPropsMetadata,
} from "../parsers/parser-ofxPropsMetadata/types.ts";
import { NameRegulator } from "../utils/name-regulator.ts";

export function genLowEnums(
  fr: FinalResultOfxPropsMetadata,
  opts: {
    nameRegulator: NameRegulator;
  },
): string {
  const parts: string[] = [
    "openfx_internal_macros::sys_helpers_make_property_enums! {",
  ];
  for (
    const [enumNameFull, variantKeyConstantSet] of Object
      .entries(fr.propEnumValues).toSorted()
  ) {
    if (!enumNameFull.startsWith("Ofx")) {
      throw new Error(
        `Unexpected enum name ${enumNameFull}. Expected to start with "Ofx".`,
      );
    }
    const enumName = enumNameFull.slice("Ofx".length);
    const variants = [...variantKeyConstantSet].toSorted()
      .map((v) => {
        const kind = getKindOfVariantName(v);
        return {
          keyConstant: v,
          kind,
          ...(kind === "key_constant"
            ? {
              canonicalName: opts.nameRegulator.keyConstantToCanonicalName(v),
            }
            : {}),
        };
      });

    parts.push(`    ${enumName} {`);

    let commonPrefix: string | null = null;
    if (variants.some((v) => v.kind === "key_constant")) {
      if (!variants.every((v) => v.kind === "key_constant")) {
        throw new Error(
          `Unexpected mix of \`simple\` and \`key_constant\` variants for enum ${enumName}.`,
        );
      }
      commonPrefix = findCommonPrefix(variants.map((v) => v.canonicalName!));
    }

    for (const variant of variants) {
      switch (variant.kind) {
        case "simple": {
          const rustVariantName = variant.keyConstant[0].toUpperCase() +
            variant.keyConstant.slice(1);
          parts.push(`        ${rustVariantName} : c"${variant.keyConstant}",`);
          break;
        }
        case "key_constant": {
          const name = opts.nameRegulator
            .keyConstantToCanonicalName(variant.keyConstant);
          const variantKName = name.slice(commonPrefix!.length);
          if (name != variant.keyConstant) {
            console.info(
              `NOTE(gen-low-enums): The enum variant with key constant \`${variant.keyConstant}\` has a different canonical name \`${name}\` for property \`${enumName}\`.`,
            );
          }
          parts.push(
            `        ${variantKName} => crate::sys_umbrella::k${name},`,
          );
          break;
        }
        default:
          variant.kind satisfies never;
      }
    }

    parts.push("    }");
  }

  parts.push("}");

  return parts.join("\n");
}

function getKindOfVariantName(name: string): "simple" | "key_constant" {
  if (/^[a-z]+$/.test(name)) {
    return "simple";
  } else if (/^(Ofx|kOfx)/.test(name)) {
    return "key_constant";
  }
  throw new Error(`Unexpected variant name ${name}.`);
}

function findCommonPrefix(strs: string[]): string {
  let i = 0;
  while (true) {
    const char = strs[0][i];
    if (char === undefined) break;
    if (!strs.every((s) => s[i] === char)) break;
    i++;
  }
  return strs[0].slice(0, i);
}
