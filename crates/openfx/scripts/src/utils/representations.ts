export function representTypeWithContainer(
  ty: string | string[],
  dimensions: number,
): string {
  if (Array.isArray(ty)) {
    ty = `(${ty.join(" | ")})`;
  }

  if (dimensions === 0) {
    return `[${ty}]`;
  } else if (dimensions === 1) {
    return ty;
  } else {
    return `[${ty}; ${dimensions}]`;
  }
}
