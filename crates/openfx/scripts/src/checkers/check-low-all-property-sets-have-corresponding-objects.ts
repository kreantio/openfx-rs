import {
  FinalResult as FinalResultOfxPropsBySet,
} from "../vibe-zone/parsers/parser-ofxPropsBySet/types.ts";
import { NameRegulator } from "../utils/name-regulator.ts";
import { CodegenConfig } from "../definitions.ts";
import { assertSetsEqual } from "../utils/assertions.ts";

export function checkLowAllPropertySetsHaveCorrespondingObjects(
  frS: FinalResultOfxPropsBySet,
  opts: {
    cfg: CodegenConfig;
    nameRegulator: NameRegulator;
  },
) {
  const propertySets = Object.keys(frS.infos.propSets)
    .map((n) => opts.nameRegulator.propertySetNameWeUse(n));

  const propertySetsSeen = [
    ...opts.cfg.objects.unused_property_sets_in_metadata,
  ];

  for (const [name, v] of Object.entries(opts.cfg.objects.mapping)) {
    if (typeof v === "string") continue;
    if (v.set) {
      propertySetsSeen.push(typeof v.set === "string" ? v.set : name);
    }
  }
  for (const [_, v] of Object.entries(opts.cfg.objects.parameter_set_mapping)) {
    propertySetsSeen.push(v.set);
  }

  assertSetsEqual(
    "Property sets",
    "those in `ofxPropsBySet.h`",
    new Set(propertySets),
    "those have corresponding objects",
    new Set(propertySetsSeen),
  );
}
