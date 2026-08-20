import { z } from "@zod/zod";

import("./impl-by-llms/mod.ts") satisfies Promise<{
  parse(headerCode: string): Result;
  makeFinalResult(result: Result): FinalResult;
}>;

export const Structure = z.tuple([
  z.literal("pragmaOnce"),
  z.literal("include[]"),
  z.object({
    type: z.literal("namespace:openfx"),
    inner: z.tuple([
      z.literal("struct:prop"),
      z.literal("static:propSets"),
      z.literal("static:actions"),
      z.literal("static:actionProps"),
      z.literal("staticAssert[]"),
    ]),
  }),
]);
export type Structure = z.infer<typeof Structure>;

export const Prop = z.object({
  propDef: z.string(),
  hostWrite: z.boolean(),
  pluginWrite: z.boolean(),
  hostOptional: z.boolean(),
});
export type Prop = z.infer<typeof Prop>;

export const PropSets = z.record(z.string(), z.record(z.string(), Prop));
export type PropSets = z.infer<typeof PropSets>;

export const Actions = z.set(z.string());
export type Actions = z.infer<typeof Actions>;

export const ActionProps = z.record(
  z.string(),
  z.object({
    inArgs: z.set(z.string()).optional(),
    outArgs: z.set(z.string()).optional(),
  }),
);
export type ActionProps = z.infer<typeof ActionProps>;

export const Infos = z.object({
  propSets: PropSets,
  actions: Actions,
  actionProps: ActionProps,
});
export type Infos = z.infer<typeof Infos>;

export const Result = z.object({
  structure: Structure,
  infos: Infos,
});
export type Result = z.infer<typeof Result>;

export const FinalResult = z.object({
  infos: Infos,
});
export type FinalResult = z.infer<typeof FinalResult>;
