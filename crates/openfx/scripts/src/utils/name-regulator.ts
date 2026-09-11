import {
  FinalResult as FinalResultOfxPropsMetadata,
} from "../vibe-zone/parsers/parser-ofxPropsMetadata/types.ts";
import {
  FinalResult as FinalResultOfxPropsBySet,
} from "../vibe-zone/parsers/parser-ofxPropsBySet/types.ts";
import { CodegenConfig } from "../definitions.ts";

export class NameRegulator {
  #kConstantToCanonicalNameMap: Record<string, string> = {};
  #actionCanonicalNameToVariantNameMap: Record<string, string> = {};
  #propertyCanonicalNameToSimpleNameRegex: RegExp;
  #propertyCanonicalNameToSimpleNameSpecialCases: Record<string, string> = {};

  constructor(opts: {
    cfg: CodegenConfig;
    propsMetadata: FinalResultOfxPropsMetadata;
    propsBySet: FinalResultOfxPropsBySet;
  }) {
    this.#kConstantToCanonicalNameMap = NameRegulator
      .#buildKConstantToCanonicalNameMap(opts);
    this.#actionCanonicalNameToVariantNameMap = NameRegulator
      .#buildActionCanonicalNameToVariantNameMap(opts);
    this.#propertyCanonicalNameToSimpleNameRegex = new RegExp(
      opts.cfg.properties.names.simple_names_regex,
    );
    this.#propertyCanonicalNameToSimpleNameSpecialCases =
      opts.cfg.properties.names.simple_names_special_cases;
  }

  static #buildKConstantToCanonicalNameMap(opts: {
    cfg: CodegenConfig;
    propsMetadata: FinalResultOfxPropsMetadata;
    propsBySet: FinalResultOfxPropsBySet;
  }): Record<string, string> {
    const map: Record<string, string> = {};
    for (
      const [constant, canonicalName] of Object.entries(
        opts.propsMetadata.keyConstantToCanonicalNameMap,
      )
    ) {
      map[constant] = canonicalName;
    }
    for (
      const [constant, canonicalName] of Object.entries(
        opts.propsBySet.keyConstantToCanonicalNameMap,
      )
    ) {
      if (map[constant]) {
        throw new Error(
          `Duplicate constant ${constant} in propsMetadata and propsBySet`,
        );
      }
      map[constant] = canonicalName;
    }
    return map;
  }

  static #buildActionCanonicalNameToVariantNameMap(opts: {
    cfg: CodegenConfig;
    propsBySet: FinalResultOfxPropsBySet;
  }): Record<string, string> {
    const map: Record<string, string> = {};

    // for (const member of opts.cfg.actions.pseudo.members) {
    //   map[member] = member;
    // }

    const actionCanonicalNames = [...opts.propsBySet.infos.actions]
      .flatMap((kConstant) => {
        const canonicalName = opts.propsBySet
          .keyConstantToCanonicalNameMap[kConstant];
        if (!canonicalName) return [];
        return [canonicalName];
      })
      .toSorted();

    for (const kind of ["core", "image_effect", "interact"] as const) {
      const regex = new RegExp(opts.cfg.actions[kind].members_regex);
      for (const canonicalName of actionCanonicalNames) {
        const g = canonicalName.match(regex);
        if (g) {
          map[canonicalName] = g[1];
        }
      }
    }
    return map;
  }

  keyConstantToCanonicalName(kConstantValue: string): string {
    const name = this.#kConstantToCanonicalNameMap[kConstantValue];
    if (!name) throw new Error(`Unknown constant: ${kConstantValue}`);
    return name;
  }

  keyConstantToKName(kConstantValue: string): string {
    return `k${this.keyConstantToCanonicalName(kConstantValue)}`;
  }

  actionCanonicalNameToVariantName(name: string): string {
    const variantName = this.#actionCanonicalNameToVariantNameMap[name];
    if (!variantName) throw new Error(`Unknown action canonical name: ${name}`);
    return variantName;
  }

  actionCanonicalNameToInArgsName(name: string, isPseudo: boolean): string {
    if (isPseudo) {
      return `${name}In`;
    } else {
      const variantName = this.actionCanonicalNameToVariantName(name);
      return `Action${variantName}In`;
    }
  }

  actionCanonicalNameToOutArgsName(name: string, isPseudo: boolean): string {
    if (isPseudo) {
      return `${name}Out`;
    } else {
      const variantName = this.actionCanonicalNameToVariantName(name);
      return `Action${variantName}Out`;
    }
  }

  propertyCanonicalNameToSimpleName(name: string): string {
    if (this.#propertyCanonicalNameToSimpleNameSpecialCases[name]) {
      return this.#propertyCanonicalNameToSimpleNameSpecialCases[name];
    }
    const m = this.#propertyCanonicalNameToSimpleNameRegex.exec(name);
    if (!m) {
      throw new Error(
        `Property name does not match regex. name=\`${name}\` regex=\`${this.#propertyCanonicalNameToSimpleNameRegex}\``,
      );
    }
    return m.slice(1).join("");
  }
}
