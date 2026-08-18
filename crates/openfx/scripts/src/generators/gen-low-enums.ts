import { CodegenConfig } from "../definitions.ts";
import {
  FinalResult as FinalResultOfxPropsMetadata,
} from "../parsers/parser-ofxPropsMetadata/types.ts";

export function genLowEnums(
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
