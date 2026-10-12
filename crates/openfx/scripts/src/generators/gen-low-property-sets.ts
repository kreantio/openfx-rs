import { snakeCase } from "es-toolkit/string";

import {
  FinalResult as FinalResultOfxPropsMetadata,
} from "../vibe-zone/parsers/parser-ofxPropsMetadata/types.ts";
import { representTypeWithContainer } from "../utils/representations.ts";
import { NameRegulator } from "../utils/name-regulator.ts";

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
