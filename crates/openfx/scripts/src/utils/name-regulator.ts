import { CodegenConfig } from "../definitions.ts";

export class PropertyNameRegulator {
  #kConstantToCanonicalNameMap: Record<string, string> = {};

  constructor(cfg: CodegenConfig) {
    for (const [k, v] of Object.entries(cfg.property_value_to_key_exceptions)) {
      this.#kConstantToCanonicalNameMap[k] = v;
    }
  }

  keyConstantToCanonicalName(value: string): string {
    return this.#kConstantToCanonicalNameMap[value] ?? value;
  }

  keyConstantToKName(value: string): string {
    return `k${this.keyConstantToCanonicalName(value)}`;
  }
}
