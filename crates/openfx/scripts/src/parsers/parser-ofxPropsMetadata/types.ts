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
      z.literal("enum:propType"),
      z.literal("enum:propId"),
      z.literal("namespace:propEnumValues"),
      z.literal("namespace:propTypeArrays"),
      z.literal("struct:propDef"),
      z.literal("struct:propDefsArray"),
      z.literal("static:propDefs"),
      z.object({
        type: z.literal("namespace:properties"),
        inner: z.tuple([
          z.literal("struct:propTraits"),
          z.literal("definePropTraits[]"),
        ]),
      }),
      z.object({
        type: z.literal("namespace:assertions"),
        inner: z.tuple([
          z.literal("using[]"),
          z.literal("staticAssert[]"),
        ]),
      }),
    ]),
  }),
]);
export type Structure = z.infer<typeof Structure>;

/**
 * Property name -> Set of possible values for that property name.
 */
export const PropEnumValues = z.record(z.string(), z.set(z.string()));
export type PropEnumValues = z.infer<typeof PropEnumValues>;

export const PropType = z.enum([
  "Int",
  "Double",
  "Enum",
  "Bool",
  "String",
  "Pointer",
]);
export type PropType = z.infer<typeof PropType>;

export const PropTypeArrays = z.record(z.string(), z.set(PropType));
export type PropTypeArrays = z.infer<typeof PropTypeArrays>;

export const PropDefsArrayItem = z.object({
  dimension: z.number().int().nonnegative(),
});
export type PropDefsArrayItem = z.infer<typeof PropDefsArrayItem>;
export const PropDefsArray = z.record(z.string(), z.array(PropDefsArrayItem));
export type PropDefsArray = z.infer<typeof PropDefsArray>;

export const PropertyInfos = z.object({
  propEnumValues: PropEnumValues,
  propTypeArrays: PropTypeArrays,
  propDefsArray: PropDefsArray,
});
export type PropertyInfos = z.infer<typeof PropertyInfos>;

export const Result = z.object({
  structure: Structure,
  propertyInfos: PropertyInfos,
});
export type Result = z.infer<typeof Result>;

export const FinalResult = z.object({
  propEnumValues: PropEnumValues,
  propertyInfos: z.record(
    z.string(),
    z.object({
      type: z.union([
        PropType.exclude(["Enum"]),
        // value: key in `PropEnumValues`
        z.record(z.literal("Enum"), z.string()),
      ]),
      dimension: z.number().int().nonnegative(),
    }),
  ),
});
export type FinalResult = z.infer<typeof FinalResult>;
