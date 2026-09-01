import {
  FinalResult as FinalResultOfxPropsMetadata,
} from "../parsers/parser-ofxPropsMetadata/types.ts";
import {
  FinalResult as FinalResultOfxPropsBySet,
} from "../parsers/parser-ofxPropsBySet/types.ts";
import { NameRegulator } from "../utils/name-regulator.ts";
import { CodegenConfig } from "../definitions.ts";
import { genPropertySetImplItem } from "./gen-low-property-sets.ts";
import { pascalCase } from "es-toolkit/string";

export function genLowActions(
  frM: FinalResultOfxPropsMetadata,
  frS: FinalResultOfxPropsBySet,
  opts: {
    cfg: CodegenConfig;
    nameRegulator: NameRegulator;
    isForPlugin: boolean;
  },
): string {
  const seenActions = new Set<string>();

  const optsEx = { ...opts, seenActions };

  return [
    genLowActionsPseudo(frM, frS, optsEx),
    genLowActionsCore(frM, frS, optsEx),
    ...(["image_effect", "interact"] as const).map((groupNameSnakeCase) =>
      genLowActionsInGroup(frM, frS, groupNameSnakeCase, optsEx)
    ),
  ].join("\n");
}

function genLowActionsPseudo(
  frM: FinalResultOfxPropsMetadata,
  frS: FinalResultOfxPropsBySet,
  opts: {
    cfg: CodegenConfig;
    nameRegulator: NameRegulator;
    seenActions: Set<string>;
    isForPlugin: boolean;
  },
) {
  const parts: string[] = ["pub mod pseudo {"];

  for (const m of opts.cfg.actions.pseudo.members) {
    opts.seenActions.add(m);

    const inName = opts.nameRegulator
      .actionCanonicalNameToInArgsName(m, true);
    const outName = opts.nameRegulator
      .actionCanonicalNameToOutArgsName(m, true);

    const argParts = [
      "handle: *const std::ffi::c_void",
      ...(frS.infos.actionProps[m].inArgs ? [`in_args: ${inName}`] : []),
      ...(frS.infos.actionProps[m].outArgs ? [`out_args: ${outName}`] : []),
    ];

    parts.push(
      `pub type ${m} = fn(${
        argParts.join(", ")
      }) -> crate::generic::low::Result<()>;`,
    );

    parts.push("openfx_internal_macros::low_make_property_set_structs! {");
    const inArgs = frS.infos.actionProps[m].inArgs;
    if (inArgs) {
      parts.push(
        genLowPropertySetStructForArguments("in", frM, inName, inArgs, opts),
      );
    }
    const outArgs = frS.infos.actionProps[m].outArgs;
    if (outArgs) {
      parts.push(
        genLowPropertySetStructForArguments("out", frM, outName, outArgs, opts),
      );
    }
    parts.push("}");
  }

  parts.push("}");

  return parts.join("\n");
}

function genLowActionsCore(
  frM: FinalResultOfxPropsMetadata,
  frS: FinalResultOfxPropsBySet,
  opts: {
    cfg: CodegenConfig;
    nameRegulator: NameRegulator;
    seenActions: Set<string>;
    isForPlugin: boolean;
  },
) {
  const membersRegex = new RegExp(opts.cfg.actions.core.members_regex);

  const parts: string[] = ["pub mod core {"];

  const actions = extractActions([...frS.infos.actions], membersRegex, opts);

  parts.push("openfx_internal_macros::low_make_property_set_structs! {");
  for (const action of actions) {
    if (opts.seenActions.has(action.canonicalName)) {
      throw new Error(
        `Duplicate action: ${action.canonicalName}`,
      );
    }
    opts.seenActions.add(action.canonicalName);

    const argSets = frS.infos.actionProps[action.canonicalName];
    if (!argSets) continue;

    const inName = opts.nameRegulator
      .actionCanonicalNameToInArgsName(action.canonicalName, false);
    const outName = opts.nameRegulator
      .actionCanonicalNameToOutArgsName(action.canonicalName, false);

    const inArgs = argSets.inArgs;
    if (inArgs) {
      parts.push(
        genLowPropertySetStructForArguments("in", frM, inName, inArgs, opts),
      );
    }
    const outArgs = argSets.outArgs;
    if (outArgs) {
      parts.push(
        genLowPropertySetStructForArguments("out", frM, outName, outArgs, opts),
      );
    }
  }
  parts.push("}");

  parts.push("}");

  return parts.join("\n");
}

function genLowActionsInGroup(
  frM: FinalResultOfxPropsMetadata,
  frS: FinalResultOfxPropsBySet,
  groupNameSnake: "image_effect" | "interact",
  opts: {
    cfg: CodegenConfig;
    nameRegulator: NameRegulator;
    seenActions: Set<string>;
    isForPlugin: boolean;
  },
) {
  const membersRegexCore = new RegExp(opts.cfg.actions.core.members_regex);
  const membersRegexSelf = new RegExp(
    opts.cfg.actions[groupNameSnake].members_regex,
  );

  const parts = [
    `pub mod ${groupNameSnake} {`,
    "    openfx_internal_macros::low_make_action_enum! {",
    `        ${pascalCase(groupNameSnake)}Action {`,
  ];

  const actionsCore = extractActions(
    opts.cfg.actions[groupNameSnake].members_from_core,
    membersRegexCore,
    opts,
  ).map((a) => ({ ...a, isFromCore: true }));
  const actionsSelf = extractActions(
    [...frS.infos.actions],
    membersRegexSelf,
    opts,
  ).map((a) => ({ ...a, isFromCore: false }));

  for (const action of [...actionsCore, ...actionsSelf]) {
    const hasInArgs = !!frS.infos.actionProps[action.canonicalName]?.inArgs;
    const hasOutArgs = !!frS.infos.actionProps[action.canonicalName]?.outArgs;

    let text = "            ";
    text += (hasInArgs ? "i" : "_") + "/";
    text += (hasOutArgs ? "o" : "_") + " ";
    if (action.isFromCore) {
      text += "core::";
    }
    text += action.variantName;

    // text += ": ";
    // text += "*const std::ffi::c_void";

    text += ",";

    parts.push(text);
  }

  parts.push("        }");
  parts.push("    }");

  parts.push("openfx_internal_macros::low_make_property_set_structs! {");
  for (const action of actionsSelf) {
    if (opts.seenActions.has(action.canonicalName)) {
      throw new Error(
        `Duplicate action: ${action.canonicalName}`,
      );
    }
    opts.seenActions.add(action.canonicalName);

    const argSets = frS.infos.actionProps[action.canonicalName];
    if (!argSets) continue;

    const inName = opts.nameRegulator
      .actionCanonicalNameToInArgsName(action.canonicalName, false);
    const outName = opts.nameRegulator
      .actionCanonicalNameToOutArgsName(action.canonicalName, false);

    const inArgs = argSets.inArgs;
    if (inArgs) {
      parts.push(
        genLowPropertySetStructForArguments("in", frM, inName, inArgs, opts),
      );
    }
    const outArgs = argSets.outArgs;
    if (outArgs) {
      parts.push(
        genLowPropertySetStructForArguments("out", frM, outName, outArgs, opts),
      );
    }
  }
  parts.push("}");

  parts.push("}");

  return parts.join("\n");
}

function genLowPropertySetStructForArguments(
  inout: "in" | "out",
  frM: FinalResultOfxPropsMetadata,
  name: string,
  args_: Set<string>,
  opts: {
    nameRegulator: NameRegulator;
    isForPlugin: boolean;
  },
) {
  const args = [...args_]
    .map((keyConstant) => ({
      keyConstant,
      name: opts.nameRegulator.keyConstantToCanonicalName(keyConstant),
    }))
    .toSorted((a, b) => a.name.localeCompare(b.name));

  const parts: string[] = [`    ${name} {`];

  for (const arg of args) {
    parts.push(genPropertySetImplItem(frM, {
      ...arg,
      readable: inout === "in",
      writable: inout === "out",
    }, opts));
  }

  parts.push("    }");

  return parts.join("\n");
}

function extractActions(actions: string[], membersRegex: RegExp, opts: {
  nameRegulator: NameRegulator;
}) {
  return actions.flatMap((action) => {
    const m = membersRegex.exec(action);
    if (!m) return [];
    return [{
      canonicalName: opts.nameRegulator.keyConstantToCanonicalName(action),
      variantName: m[1],
    }];
  });
}
