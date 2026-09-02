import { snakeCase } from "es-toolkit/string";

import {
  FinalResult as FinalResultOfxPropsMetadata,
} from "../parsers/parser-ofxPropsMetadata/types.ts";
import {
  FinalResult as FinalResultOfxPropsBySet,
} from "../parsers/parser-ofxPropsBySet/types.ts";
import { representTypeWithContainer } from "../utils/representations.ts";
import { NameRegulator } from "../utils/name-regulator.ts";
import { CodegenConfig } from "../definitions.ts";

export function genLowPropertySets(
  frM: FinalResultOfxPropsMetadata,
  frS: FinalResultOfxPropsBySet,
  opts: {
    cfg: CodegenConfig;
    nameRegulator: NameRegulator;
    isForPlugin: boolean;
  },
): string {
  const parts: string[] = [
    "openfx_internal_macros::low_make_property_set_structs! {",
  ];

  for (const [name, set] of Object.entries(frS.infos.propSets)) {
    parts.push(`    ${name} {`);

    const props = Object.entries(set)
      .map(([propName, prop]): PropertyItem => {
        if (prop.pluginWrite === prop.hostWrite) {
          throw new Error(
            `Property ${propName} has the same pluginWrite and hostWrite value: ${prop.pluginWrite}`,
          );
        }

        const writable = opts.isForPlugin ? prop.pluginWrite : prop.hostWrite;

        return {
          keyConstant: propName,
          name: opts.nameRegulator.keyConstantToCanonicalName(propName),
          readable: !writable,
          writable: writable,
        };
      })
      .toSorted((a, b) => a.name.localeCompare(b.name, "en"));

    for (const prop of props) {
      parts.push(
        genPropertySetPropertyItem(frM, prop, opts),
      );
    }

    parts.push("    }");
  }

  parts.push("}");

  return parts.join("\n");
}

interface PropertyItem {
  keyConstant: string;
  name: string;
  readable: boolean;
  writable: boolean;
}

export function genPropertySetPropertyItem(
  frM: FinalResultOfxPropsMetadata,
  prop: PropertyItem,
  opts: { nameRegulator: NameRegulator; isForPlugin: boolean },
): string {
  const def = frM.propertyInfos[prop.keyConstant];
  if (!def) {
    throw new Error(
      `Missing property info for key constant: ${prop.keyConstant}`,
    );
  }

  const nameSimple = opts.nameRegulator
    .propertyCanonicalNameToSimpleName(prop.name);
  const nameSnake = snakeCase(nameSimple);
  const nameSnakeSafe = sanitizeSnakeName(nameSnake);

  const vTy = ((): string | string[] => {
    if (def.type instanceof Set) {
      return [...def.type].toSorted();
    } else if (typeof def.type === "object") {
      def.type satisfies { "Enum": unknown };
      const canonicName = opts.nameRegulator
        .keyConstantToCanonicalName(def.type.Enum);
      const name = canonicName.slice("Ofx".length);
      return `Enum(${name})`;
    } else {
      return def.type;
    }
  })();
  const tyContainer = representTypeWithContainer(vTy, def.dimension);
  const access = [prop.readable ? "r" : "_", prop.writable ? "w" : "_"].join(
    "/",
  );
  return "        " +
    `${access} ${nameSnakeSafe}: ${tyContainer} @${prop.name};`;
}

const rustKeywords = new Set(["type"]);

function sanitizeSnakeName(nameSnake: string): string {
  if (rustKeywords.has(nameSnake)) {
    return `r#${nameSnake}`;
  }
  return nameSnake;
}
