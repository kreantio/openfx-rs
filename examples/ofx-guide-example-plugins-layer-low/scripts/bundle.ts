// deno-lint-ignore-file no-explicit-any

import path from "node:path";

import { parseArgs } from "@std/cli/parse-args";

import $ from "@david/dax";

async function doParseArgs(args: string[]) {
  const result = parseArgs(args, {
    string: [
      "target-triple",
      "base-name",
      "info-plist",
    ],
    default: {
      "target-triple": await fetchCurrentTargetTriple(),
    },
  });

  if (!result["base-name"]) throw new Error("Missing `--base-name`");
  if (!result["info-plist"]) throw new Error("Missing `--info-plist`");

  return result;
}
type Args = Awaited<ReturnType<typeof doParseArgs>>;

async function main(args: Args) {
  const targetTriple = args["target-triple"];
  const baseName = args["base-name"]!;
  const infoPlistPath = args["info-plist"]!;

  const target = TARGETS.find((t) => t.targetTriple === targetTriple);
  if (!target) throw new Error(`Unsupported target triple: ${targetTriple}`);

  const metadata = new Metadata(
    await $`cargo metadata --format-version 1 --no-deps --offline`.json(),
    target,
  );

  console.info(`Building for target \`${targetTriple}\`…`);
  await $`cargo build --release --target ${targetTriple}`;

  console.info(`Bundling…`);
  bundle(metadata, { baseName, infoPlistPath });

  console.info(`Done!`);
}

async function bundle(metadata: Metadata, opts: {
  baseName: string;
  infoPlistPath: string;
}) {
  const cratePath = path.dirname(await metadata.fetchCrateManifestPath());
  const libBinPath = await metadata.makeLibraryBinaryPath();

  const outDirPath = `${cratePath}/build/${metadata.target.ofxArchitecture}`;
  const bundleContentsPath = path
    .join(outDirPath, `${opts.baseName}.ofx.bundle`, "Contents");

  await $`mkdir -p ${outDirPath}`;
  await $`mkdir -p ${
    path.join(bundleContentsPath, metadata.target.ofxArchitecture)
  }`;
  await $`mkdir -p ${path.join(bundleContentsPath, "Resources")}`;

  await $`cp ${libBinPath} ${
    path.join(
      bundleContentsPath,
      metadata.target.ofxArchitecture,
      `${opts.baseName}.ofx`,
    )
  }`;
  await $`cp ${opts.infoPlistPath} ${
    path.join(bundleContentsPath, "Info.plist")
  }`;
}

class Metadata {
  constructor(
    public raw: any,
    public target: Target,
  ) {}

  #crateManifestPath: string | undefined;
  async fetchCrateManifestPath() {
    return this.#crateManifestPath ??=
      await $`cargo locate-project --message-format plain`.text();
  }

  #workspaceRootPath: string | undefined;
  async fetchWorkspaceRootManifestPath() {
    return this.#workspaceRootPath ??=
      await $`cargo locate-project --workspace --message-format plain`.text();
  }

  #currentPackage: any | undefined;
  async mustFetchCurrentPackage() {
    return this.#currentPackage ??= await (async () => {
      const cargoManifestPath = await Deno.realPath(
        await this.fetchCrateManifestPath(),
      );

      const curPkg = (this.raw.packages as any[])
        .find((p) => Deno.realPathSync(p.manifest_path) === cargoManifestPath);
      if (!curPkg) throw new Error("Could not find current package");

      return curPkg;
    })();
  }

  async findLibraryTargetForCurrentPackage() {
    const curPkg = await this.mustFetchCurrentPackage();
    return curPkg.targets.find((t: any) => t.kind.includes("cdylib"));
  }

  async makeLibraryBinaryPath() {
    const workspaceManifestPath = await this.fetchWorkspaceRootManifestPath();
    const workspaceRootPath = path.dirname(workspaceManifestPath);
    const lib = await this.findLibraryTargetForCurrentPackage();
    return `${workspaceRootPath}/target/${this.target.targetTriple}/release/${this.target.libraryPrefix}${lib.name}.${this.target.libraryExtension}`;
  }
}

let currentTargetTriple: string | undefined;
async function fetchCurrentTargetTriple() {
  // https://stackoverflow.com/a/69816610
  return currentTargetTriple ??= await $`rustc -vV | sed -n 's|host: ||p'`
    .text();
}

interface Target {
  targetTriple: string;
  ofxArchitecture: string;
  libraryExtension: string;
  libraryPrefix: string;
}
/**
 * took from: https://github.com/ntsc-rs/ntsc-rs/blob/af9833b4bb81f195f7fe4a3667211f2a94139a42/xtask/src/util/targets.rs
 *
 * - license: MIT (Copyright © valadaptive)
 * - license link: https://github.com/ntsc-rs/ntsc-rs/blob/af9833b4bb81f195f7fe4a3667211f2a94139a42/LICENSE-MIT
 */
const TARGETS: Target[] = [
  {
    targetTriple: "x86_64-unknown-linux-gnu",
    ofxArchitecture: "Linux-x86-64",
    libraryExtension: "so",
    libraryPrefix: "lib",
  },
  {
    targetTriple: "i686-unknown-linux-gnu",
    ofxArchitecture: "Linux-x86",
    libraryExtension: "so",
    libraryPrefix: "lib",
  },
  {
    targetTriple: "x86_64-pc-windows-msvc",
    ofxArchitecture: "Win64",
    libraryExtension: "dll",
    libraryPrefix: "",
  },
  {
    targetTriple: "i686-pc-windows-msvc",
    ofxArchitecture: "Win32",
    libraryExtension: "dll",
    libraryPrefix: "",
  },
  {
    targetTriple: "x86_64-apple-darwin",
    ofxArchitecture: "MacOS",
    libraryExtension: "dylib",
    libraryPrefix: "lib",
  },
  {
    targetTriple: "aarch64-apple-darwin",
    ofxArchitecture: "MacOS",
    libraryExtension: "dylib",
    libraryPrefix: "lib",
  },
];

await main(await doParseArgs(Deno.args));
