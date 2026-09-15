export function assertSetsEqual(
  forWhat: string,
  aName: string,
  a: Set<string>,
  bName: string,
  b: Set<string>,
) {
  if (a.size === b.size && a.isSubsetOf(b)) return;

  const onlyInA = a.difference(b);
  const onlyInB = b.difference(a);
  let err = `${forWhat} in ${aName} and ${bName} differ:`;
  if (onlyInA.size > 0) {
    err += ` some members are only in ${aName} (${
      Array.from(onlyInA).join(", ")
    })`;
  }
  if (onlyInB.size > 0) {
    if (onlyInA.size > 0) {
      err += ", while";
    }
    err += ` some members are only in ${bName} (${
      Array.from(onlyInB).join(", ")
    })`;
  }
  err += ".";

  throw new Error(err);
}
