import {
  FinalResult as FinalResultOfxPropsMetadata,
} from "../vibe-zone/parsers/parser-ofxPropsMetadata/types.ts";
import {
  FinalResult as FinalResultOfxPropsBySet,
} from "../vibe-zone/parsers/parser-ofxPropsBySet/types.ts";
import { NameRegulator } from "../utils/name-regulator.ts";
import { CodegenConfig } from "../definitions.ts";
import { genPropertySetPropertyItem } from "./gen-low-property-sets.ts";
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

  const parts = [
    genLowActionsPseudo(frM, frS, optsEx),
    genLowActionsCore(frM, frS, optsEx),
    ...(["image_effect", "interact"] as const).map((groupNameSnakeCase) =>
      genLowActionsInGroup(frM, frS, groupNameSnakeCase, optsEx)
    ),
  ];

  if (seenActions.size < frS.infos.actions.size) {
    throw new Error(
      `Some actions were not seen: ${[
        ...frS.infos.actions.difference(seenActions),
      ]}`,
    );
  }

  return parts.join("\n");
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
      `pub type ${m} = fn(${argParts.join(", ")}) -> crate::low::Result<()>;`,
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

  const parts: string[] = [
    "pub mod core {",
    "openfx_internal_macros::low_make_property_set_structs! {",
  ];

  const actions = extractActions([...frS.infos.actions], membersRegex, opts);

  verifyActionMembers(
    "core",
    new Set(actions.map((a) => a.variantName)),
    new Set(Object.keys(opts.cfg.actions.core.members)),
  );

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

  verifyActionMembers(
    groupNameSnake,
    new Set(actionsSelf.map((a) => a.variantName)),
    new Set(Object.keys(opts.cfg.actions[groupNameSnake].members)),
  );

  for (const action of [...actionsCore, ...actionsSelf]) {
    const hasInArgs = !!frS.infos.actionProps[action.canonicalName]?.inArgs;
    const hasOutArgs = !!frS.infos.actionProps[action.canonicalName]?.outArgs;
    const actionConfig = action.isFromCore
      ? opts.cfg.actions.core.members[action.variantName]!
      : opts.cfg.actions[groupNameSnake].members[action.variantName]!;

    if (actionConfig.in !== hasInArgs) {
      console.warn(
        `Mismatch in 'in' argument expectation for action ${action.canonicalName}: config expects ${actionConfig.in}, but actual hasInArgs is ${hasInArgs}`,
      );
    }
    if (actionConfig.out !== hasOutArgs) {
      console.warn(
        `Mismatch in 'out' argument expectation for action ${action.canonicalName}: config expects ${actionConfig.out}, but actual hasOutArgs is ${hasOutArgs}`,
      );
    }

    let text = "            ";
    text += (hasInArgs ? "i" : "_") + "/";
    text += (hasOutArgs ? "o" : "_") + " ";
    if (action.isFromCore) {
      text += "core::";
    }
    text += action.variantName;

    if (actionConfig.handle) {
      text += ": " + "crate::low_plugin::objects::" +
        opts.cfg.actions[groupNameSnake].handle_prefix + actionConfig.handle;
    }

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
    .toSorted((a, b) => a.name.localeCompare(b.name, "en"));

  const parts: string[] = [`    ${name} {`];

  for (const arg of args) {
    parts.push(genPropertySetPropertyItem(frM, {
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

function verifyActionMembers(
  groupNameSnake: string,
  membersFromRegex: Set<string>,
  membersFromConfig: Set<string>,
) {
  if (
    membersFromRegex.size === membersFromConfig.size &&
    membersFromRegex.isSubsetOf(membersFromConfig)
  ) {
    return;
  }

  const onlyInConfig = membersFromConfig.difference(membersFromRegex);
  const onlyMatchedByRegex = membersFromRegex.difference(membersFromConfig);
  let err =
    `Action members in the configuration and matched by regex differ in group "${groupNameSnake}":`;
  if (onlyInConfig.size > 0) {
    err += ` some members are only in the configuration (${
      Array.from(onlyInConfig).join(", ")
    })`;
  }
  if (onlyMatchedByRegex.size > 0) {
    if (onlyInConfig.size > 0) {
      err += ", while";
    }
    err += ` some members are only matched by regex (${
      Array.from(onlyMatchedByRegex).join(", ")
    })`;
  }
  err += ".";

  throw new Error(err);
}
