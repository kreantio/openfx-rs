import {
  FinalResult as FinalResultOfxPropsMetadata,
} from "../parsers/parser-ofxPropsMetadata/types.ts";
import { PropertyNameRegulator } from "../utils/name-regulator.ts";

export function genLowEnums(
  fr: FinalResultOfxPropsMetadata,
  opts: {
    propertyNameRegulator: PropertyNameRegulator;
  },
): string {
  const items: string[] = [];
  for (
    const [enumName, variantKeyConstantSet] of Object.entries(fr.propEnumValues)
      .toSorted((a, b) => a[0].localeCompare(b[0]))
  ) {
    const variantKeyConstants = [...variantKeyConstantSet];
    const isSimple = variantKeyConstants.every((v) => v === v.toLowerCase());
    if (isSimple) {
      variantKeyConstants.sort();
      items.push(
        `crate::internal::low_macros::make_enum_from_idents!(${enumName},${
          variantKeyConstants.map((v) => `\n    r#${v} : c"${v}"`).join(", ")
        }\n);`,
      );
    } else {
      const variantCanonicalNames = variantKeyConstants.map((v) => {
        const name = opts.propertyNameRegulator.keyConstantToCanonicalName(v);
        if (name != v) {
          console.info(
            `NOTE(gen-low-enums): The enum variant with key constant \`${v}\` has a different canonical name \`${name}\` for property \`${enumName}\`.`,
          );
        }
        return name;
      }).toSorted();
      items.push(
        `crate::internal::low_macros::make_enum_from_paths!(${enumName},${
          variantCanonicalNames.map((v) => {
            const path = `crate::sys_umbrella::k${v}`;
            return `\n    /// See: [\`${path}\`].\n    ${v} => ${path}`;
          }).join(", ")
        }\n);`,
      );
    }
  }

  return items.join("\n");
}
