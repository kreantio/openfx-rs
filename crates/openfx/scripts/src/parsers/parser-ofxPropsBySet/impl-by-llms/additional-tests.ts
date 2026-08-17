// Strictness and data-extraction tests for the `ofxPropsBySet.h` parser.
//
// - `parses the real header: data spot checks` pins down facts extracted from
//   the vendored header, so any structural change in it fails here.
// - `rejects structural mutations` feeds a minimal valid header (and damaged
//   variants of it) through `parse`; every mutation must be rejected. The
//   unmutated snippet is a positive control.

import { assert, assertEquals, assertThrows } from "@std/assert";

import { makeFinalResult, parse } from "./mod.ts";

/** The vendored header, resolved relative to this file (cwd-independent). */
const realHeaderUrl = new URL(
  "../../../../../vendor/openfx/openfx-cpp/include/openfx/ofxPropsBySet.h",
  import.meta.url,
);

/**
 * A minimal header exercising every grammar region, obeying every invariant
 * the parser enforces (ascending orders, name correspondences, the declared
 * actions count, assert order).
 */
const validSnippet = `
namespace openfx {

struct Prop {
  const char *name;
  Prop(const char *n) : name(n) {}
};

static inline const std::map<const char *, std::vector<Prop>> prop_sets {
{ "Alpha", {
   { "OfxPropType", prop_defs[PropId::OfxPropType], false, true, false },
   { "kOfxExtra", prop_defs[PropId::OfxExtra], true, false, true } } },
{ "Beta", {
   { "OfxPropName", prop_defs[PropId::OfxPropName], false, true, false } } },
};

static inline const std::array<const char *, 2> actions {
  "OfxActionLoad",
  "OfxActionRender",
};

static inline const std::map<std::array<std::string_view, 2>, std::vector<const char *>> action_props {
{ { "OfxActionLoad", "inArgs" },
  { "OfxPropTime" } },
{ { "OfxActionRender", "outArgs" },
  { "OfxPropA",
    "OfxPropB" } },
};

static_assert(std::string_view("OfxActionLoad") == std::string_view(kOfxActionLoad));
static_assert(std::string_view("OfxActionRender") == std::string_view(kOfxActionRender));
} // namespace openfx
`;

const preamble = `#pragma once
#include <string>
#include "ofxPropsMetadata.h"
`;

function snippet(): string {
  return preamble + validSnippet;
}

Deno.test("parses a minimal header", () => {
  const result = parse(snippet());
  assertEquals(Object.keys(result.infos.propSets), ["Alpha", "Beta"]);
  assertEquals(result.infos.propSets["Alpha"]!["OfxPropType"], {
    propDef: "OfxPropType",
    hostWrite: false,
    pluginWrite: true,
    hostOptional: false,
  });
  // The `k`-prefixed name maps to the un-prefixed PropId member.
  assertEquals(
    result.infos.propSets["Alpha"]!["kOfxExtra"]!.propDef,
    "OfxExtra",
  );
  assertEquals(
    [...result.infos.actions],
    ["OfxActionLoad", "OfxActionRender"],
  );
  assertEquals(
    result.infos.actionProps["OfxActionLoad"]!.inArgs,
    new Set([
      "OfxPropTime",
    ]),
  );
  assertEquals(
    result.infos.actionProps["OfxActionRender"]!.outArgs,
    new Set([
      "OfxPropA",
      "OfxPropB",
    ]),
  );
});

Deno.test("rejects structural mutations", async (t) => {
  const cases: [string, () => string][] = [
    ["no pragma once", () => snippet().replace("#pragma once\n", "")],
    ["no includes", () => validSnippet],
    ["unknown directive", () => snippet() + "#define N 1\n"],
    [
      "renamed namespace",
      () => snippet().replace("namespace openfx", "namespace other"),
    ],
    [
      "renamed prop_sets variable",
      () => snippet().replace(">> prop_sets {", ">> propSetList {"),
    ],
    [
      "renamed struct",
      () => snippet().replace("struct Prop {", "struct Property {"),
    ],
    [
      "sections reordered",
      () => {
        const src = snippet();
        const start = src.indexOf("static inline const std::array");
        const end = src.indexOf("};", start) + 2;
        const actions = src.slice(start, end);
        const without = src.slice(0, start) + src.slice(end);
        return without.replace(
          "static inline const std::map<const char *, std::vector<Prop>>",
          actions +
            "\n\nstatic inline const std::map<const char *, std::vector<Prop>>",
        );
      },
    ],
    [
      "declared actions count too high",
      () =>
        snippet().replace(
          "const char *, 2> actions",
          "const char *, 3> actions",
        ),
    ],
    [
      "extra actions element",
      () =>
        snippet().replace(
          `  "OfxActionRender",`,
          `  "OfxActionRender",\n  "OfxActionZzz",`,
        ),
    ],
    [
      "actions not ascending",
      () =>
        snippet().replace(`"OfxActionLoad"`, `"OfxActionRender"`).replace(
          `"OfxActionRender",\n};`,
          `"OfxActionLoad",\n};`,
        ),
    ],
    [
      "prop set not ascending",
      () => snippet().replace(`{ "Beta", {`, `{ "Aardvark", {`),
    ],
    [
      "duplicate prop set",
      () => snippet().replace(`{ "Beta", {`, `{ "Alpha", {`),
    ],
    [
      "prop name unrelated to PropId",
      () =>
        snippet().replace(
          `"OfxPropName", prop_defs[PropId::OfxPropName]`,
          `"OfxPropName", prop_defs[PropId::SomethingElse]`,
        ),
    ],
    [
      "prop flags are integers",
      () =>
        snippet().replace(
          'false, true, false },\n   { "kOfxExtra"',
          '0, 1, 0 },\n   { "kOfxExtra"',
        ),
    ],
    [
      "last prop set pair lacks trailing comma",
      () =>
        snippet().replace(
          `{ "OfxPropName", prop_defs[PropId::OfxPropName], false, true, false } } },\n};`,
          `{ "OfxPropName", prop_defs[PropId::OfxPropName], false, true, false } } }\n};`,
        ),
    ],
    [
      "action_props references unknown action",
      () =>
        snippet().replace(
          `{ "OfxActionLoad", "inArgs" }`,
          `{ "OfxNoSuchAction", "inArgs" }`,
        ),
    ],
    [
      "action_props kind invalid",
      () => snippet().replace(`"inArgs" }`, `"sideArgs" }`),
    ],
    [
      "action_props values not ascending",
      () =>
        snippet().replace(
          `  { "OfxPropA",\n    "OfxPropB" } },`,
          `  { "OfxPropB",\n    "OfxPropA" } },`,
        ),
    ],
    [
      "static_assert identifier mismatch",
      () =>
        snippet().replace(
          `== std::string_view(kOfxActionRender)`,
          `== std::string_view(kOfxSomethingElse)`,
        ),
    ],
    [
      "static_assert not in actions",
      () =>
        snippet().replace(
          `static_assert(std::string_view("OfxActionRender")`,
          `static_assert(std::string_view("OfxUnassertedAction")`,
        ),
    ],
    [
      "static_asserts out of actions order",
      () => {
        const src = snippet();
        const a =
          `static_assert(std::string_view("OfxActionLoad") == std::string_view(kOfxActionLoad));\n`;
        const b =
          `static_assert(std::string_view("OfxActionRender") == std::string_view(kOfxActionRender));\n`;
        return src.replace(a + b, b + a);
      },
    ],
    [
      "no static_asserts",
      () => {
        const src = snippet();
        return src.slice(0, src.indexOf("static_assert"));
      },
    ],
    [
      "stray declaration in namespace",
      () =>
        snippet().replace(
          "} // namespace openfx",
          "int stray;\n} // namespace openfx",
        ),
    ],
  ];

  for (const [name, mutate] of cases) {
    await t.step(name, () => {
      assertThrows(() => parse(mutate()), Error);
    });
  }
});

Deno.test("parses the real header: data spot checks", async (t) => {
  const headerCode = await Deno.readTextFile(realHeaderUrl);
  const result = parse(headerCode);

  await t.step("propSets", () => {
    // 21 sets, in header (ascending) order.
    assertEquals(Object.keys(result.infos.propSets).length, 21);
    assertEquals(Object.keys(result.infos.propSets)[0], "ClipDescriptor");
    assertEquals(Object.keys(result.infos.propSets).at(-1), "ParamsString");
    assertEquals(result.infos.propSets["ClipDescriptor"]!["OfxPropType"], {
      propDef: "OfxPropType",
      hostWrite: false,
      pluginWrite: true,
      hostOptional: false,
    });
    assertEquals(result.infos.propSets["Image"]!["OfxImagePropData"], {
      propDef: "OfxImagePropData",
      hostWrite: true,
      pluginWrite: false,
      hostOptional: false,
    });
    // `k`-prefixed name maps to the un-prefixed PropId member.
    assertEquals(
      result.infos
        .propSets["ParamDouble1D"]!["kOfxParamPropUseHostOverlayHandle"]!
        .propDef,
      "OfxParamPropUseHostOverlayHandle",
    );
    // A duplicated prop name inside one set: the record keeps the last.
    assertEquals(
      result.infos.propSets["ParamsParametric"]!["OfxParamPropIsAnimating"],
      {
        propDef: "OfxParamPropIsAnimating",
        hostWrite: true,
        pluginWrite: false,
        hostOptional: false,
      },
    );
  });

  await t.step("actions", () => {
    assertEquals(result.infos.actions.size, 33);
    assert(result.infos.actions.has("CustomParamInterpFunc"));
    assert(result.infos.actions.has("OfxInteractActionPenUp"));
  });

  await t.step("actionProps", () => {
    // 28 entries over 24 actions (CustomParamInterpFunc, GetFramesNeeded,
    // GetOutputColourspace, and GetRegionOfDefinition have both kinds).
    assertEquals(Object.keys(result.infos.actionProps).length, 24);
    assertEquals(result.infos.actionProps["CustomParamInterpFunc"], {
      inArgs: new Set([
        "OfxParamPropCustomValue",
        "OfxParamPropInterpolationAmount",
        "OfxParamPropInterpolationTime",
      ]),
      outArgs: new Set([
        "OfxParamPropCustomValue",
        "OfxParamPropInterpolationTime",
      ]),
    });
    assertEquals(
      result.infos.actionProps["OfxActionInstanceChanged"]!.inArgs,
      new Set([
        "OfxImageEffectPropRenderScale",
        "OfxImageEffectPropThumbnailRender",
        "OfxPropChangeReason",
        "OfxPropName",
        "OfxPropTime",
        "OfxPropType",
      ]),
    );
    // The one outArgs entry whose values repeat across kinds.
    assertEquals(
      result.infos.actionProps["OfxImageEffectActionGetOutputColourspace"],
      {
        inArgs: new Set(["OfxImageClipPropPreferredColourspaces"]),
        outArgs: new Set(["OfxImageClipPropColourspace"]),
      },
    );
    // A value vector that lists a duplicate in the header: the Set keeps one.
    assertEquals(
      result.infos.actionProps["OfxImageEffectActionBeginSequenceRender"]!
        .inArgs!
        .has("OfxImageEffectPropInteractiveRenderStatus"),
      true,
    );
  });

  await t.step("final result passthrough", () => {
    const finalResult = makeFinalResult(result);
    assertEquals(finalResult.infos, result.infos);
  });
});
