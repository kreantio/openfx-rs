import {
  FinalResult as FinalResultOfxPropsMetadata,
} from "../parsers/parser-ofxPropsMetadata/types.ts";
import { NameRegulator } from "../utils/name-regulator.ts";

export function genLowEnums(
  fr: FinalResultOfxPropsMetadata,
  opts: {
    propertyNameRegulator: NameRegulator;
  },
): string {
  const items: string[] = [];
  for (
    const [enumNameFull, variantKeyConstantSet] of Object.entries(
      fr.propEnumValues,
    )
      .toSorted((a, b) => a[0].localeCompare(b[0]))
  ) {
    if (!enumNameFull.startsWith("Ofx")) {
      throw new Error(
        `Unexpected enum name ${enumNameFull}. Expected to start with "Ofx".`,
      );
    }
    const enumName = enumNameFull.slice("Ofx".length);
    const variantKeyConstants = [...variantKeyConstantSet];

    const kind = getKindOfVariantKeys(variantKeyConstants);

    switch (kind) {
      case "simple": {
        items.push(
          `crate::internal::low_macros::make_enum_from_idents!(${enumName},`,
        );

        variantKeyConstants.sort();
        for (const v of variantKeyConstants) {
          const rustVariantName = v[0].toUpperCase() + v.slice(1);
          items.push(`    ${rustVariantName} : c"${v}",`);
        }

        items.push(");");
        break;
      }

      case "key_constant": {
        items.push(
          `crate::internal::low_macros::make_enum_from_paths!(${enumName},`,
        );

        const names = variantKeyConstants.map((v) => {
          const name = opts.propertyNameRegulator.keyConstantToCanonicalName(v);
          if (name != v) {
            console.info(
              `NOTE(gen-low-enums): The enum variant with key constant \`${v}\` has a different canonical name \`${name}\` for property \`${enumName}\`.`,
            );
          }
          return name;
        }).toSorted();
        const namesWithoutPrefix = removeCommonPrefix(names);

        for (const i in names) {
          const [name, nameWithoutPrefix] = [names[i], namesWithoutPrefix[i]];
          const path = `crate::sys_umbrella::k${name}`;
          items.push(
            `    /// See: [\`${path}\`].\n    ${nameWithoutPrefix} => ${path},`,
          );
        }

        items.push(");");
        break;
      }

      default:
        kind satisfies never;
    }
  }

  return items.join("\n");
}

function getKindOfVariantKeys(keys: string[]): "simple" | "key_constant" {
  if (keys.every((v) => /^[a-z]+$/.test(v))) {
    return "simple";
  } else if (keys.every((v) => v.startsWith("Ofx") || v.startsWith("kOfx"))) {
    return "key_constant";
  }
  throw new Error(`Unexpected variant keys: ${keys.join(", ")}`);
}

function removeCommonPrefix(strs: string[]): string[] {
  let i = 0;
  while (true) {
    const char = strs[0][i];
    if (char === undefined) break;
    if (!strs.every((s) => s[i] === char)) break;
    i++;
  }
  return strs.map((s) => s.slice(i));
}
