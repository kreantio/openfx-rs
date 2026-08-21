import {
  FinalResult as FinalResultOfxPropsMetadata,
} from "../parsers/parser-ofxPropsMetadata/types.ts";
import {
  FinalResult as FinalResultOfxPropsBySet,
} from "../parsers/parser-ofxPropsBySet/types.ts";

export class NameRegulator {
  #kConstantToCanonicalNameMap: Record<string, string> = {};

  constructor(opts: {
    propsMetadata: FinalResultOfxPropsMetadata;
    propsBySet: FinalResultOfxPropsBySet;
  }) {
    for (
      const [constant, canonicalName] of Object.entries(
        opts.propsMetadata.keyConstantToCanonicalNameMap,
      )
    ) {
      this.#kConstantToCanonicalNameMap[constant] = canonicalName;
    }
    for (
      const [constant, canonicalName] of Object.entries(
        opts.propsBySet.keyConstantToCanonicalNameMap,
      )
    ) {
      if (this.#kConstantToCanonicalNameMap[constant]) {
        throw new Error(
          `Duplicate constant ${constant} in propsMetadata and propsBySet`,
        );
      }
      this.#kConstantToCanonicalNameMap[constant] = canonicalName;
    }
  }

  keyConstantToCanonicalName(value: string): string {
    const name = this.#kConstantToCanonicalNameMap[value];
    if (!name) throw new Error(`Unknown constant: ${value}`);
    return name;
  }

  keyConstantToKName(value: string): string {
    return `k${this.keyConstantToCanonicalName(value)}`;
  }
}
