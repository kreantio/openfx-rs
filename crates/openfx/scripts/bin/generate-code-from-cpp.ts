import path from "node:path";

import { parseArgs } from "@std/cli/parse-args";

import * as toml from "toml";

import {
  makeFinalResult as makeFinalResultOfxPropsMetadata,
  parse as parseOfxPropsMetadata,
} from "../src/parsers/parser-ofxPropsMetadata/impl-by-llms/mod.ts";
import {
  FinalResult as FinalResultOfxPropsMetadata,
} from "../src/parsers/parser-ofxPropsMetadata/types.ts";

function doParseArgs(args: string[]) {
  const result = parseArgs(args, {
    string: [
      "codegen-config",
      "input-cpp-headers",
      "output-code-from-cpp",
    ],
  });

  if (!result["codegen-config"]) {
    throw new Error("Missing `--codegen-config`");
  }
  if (!result["input-cpp-headers"]) {
    throw new Error("Missing `--input-cpp-headers`");
  }
  if (!result["output-code-from-cpp"]) {
    throw new Error("Missing `--output-code-from-cpp`");
  }

  return result;
}
type _Args = ReturnType<typeof doParseArgs>;
type Args = Required<_Args>;

interface CodegenConfig {
  "property_value_to_key_exceptions": Record<string, string>;
}

function parseCodegenConfig(tomlText: string): CodegenConfig {
  return toml.parse(tomlText);
}

async function main(args: Args) {
  const codegenConfig = parseCodegenConfig(
    await Deno.readTextFile(args["codegen-config"]),
  );

  const propsMetadata = makeFinalResultOfxPropsMetadata(parseOfxPropsMetadata(
    await Deno.readTextFile(
      path.join(args["input-cpp-headers"], "ofxPropsMetadata.h"),
    ),
  ));

  await Deno.writeTextFile(
    path.join(args["output-code-from-cpp"], "low_enums.rs"),
    genLowEnums(propsMetadata, codegenConfig),
  );
}

function genLowEnums(
  fr: FinalResultOfxPropsMetadata,
  cfg: CodegenConfig,
): string {
  const items: string[] = [];
  items.push(
    `
    macro make_enum_from_idents($name:ident, $($var:ident : $var_cstr:literal),*) {
      #[derive(Debug, Clone)]
      pub enum $name {
        $($var,)*
        Other(*const std::os::raw::c_char),
      }
      impl $name {
        /// ## SAFETY
        ///
        /// - The pointer must be valid and point to a null-terminated C string.
        /// - The pointer must live at least as long as the returned [\`Self\`] value.
        pub unsafe fn from_ptr(ptr: *const std::os::raw::c_char) -> Self {
          let cstr = unsafe { std::ffi::CStr::from_ptr(ptr) };
          $(
            if cstr == $var_cstr {
              return Self::$var;
            }
          )*
          Self::Other(cstr.as_ptr())
        }

        /// ## SAFETY
        ///
        /// The returned pointer is valid as long as the [\`std::ffi::CString\`] inside [\`Self::Other\`] is not dropped.
        pub fn as_ptr(&self) -> *const std::os::raw::c_char {
          if let Self::Other(ptr) = self {
            return *ptr;
          }
          $(
            if matches!(self, Self::$var) {
              return $var_cstr.as_ptr() as *const std::os::raw::c_char;
            }
          )*
          unreachable!()
        }
      }
    }`.replaceAll(/^ {4}/gm, "") + "\n",
  );
  items.push(
    `
    macro make_enum_from_paths($name:ident, $(#[$meta:meta] $var:ident => $var_path:path),*) {
      #[derive(Debug, Clone)]
      pub enum $name {
        $(#[$meta] $var,)*
        Other(*const std::os::raw::c_char),
      }
      impl $name {
        /// ## SAFETY
        ///
        /// - The pointer must be valid and point to a null-terminated C string.
        /// - The pointer must live at least as long as the returned [\`Self\`] value.
        pub unsafe fn from_ptr(ptr: *const std::os::raw::c_char) -> Self {
          let cstr = unsafe { std::ffi::CStr::from_ptr(ptr) };
          $(
            if cstr == $var_path {
              return Self::$var;
            }
          )*
          Self::Other(cstr.as_ptr())
        }

        /// ## SAFETY
        ///
        /// The returned pointer is valid as long as the [\`std::ffi::CString\`] inside [\`Self::Other\`] is not dropped.
        pub fn as_ptr(&self) -> *const std::os::raw::c_char {
          if let Self::Other(ptr) = self {
            return *ptr;
          }
          $(
            if matches!(self, Self::$var) {
              return $var_path.as_ptr() as *const std::os::raw::c_char;
            }
          )*
          unreachable!()
        }
      }
    }`.replaceAll(/^ {4}/gm, "") + "\n",
  );

  for (
    const [name, values] of Object.entries(fr.propEnumValues)
      .toSorted((a, b) => a[0].localeCompare(b[0]))
  ) {
    const valuesList = [...values]
      .map((v) => {
        const fix = cfg.property_value_to_key_exceptions[v];
        if (fix) {
          console.info(
            `Fix: replacing enum variant name "${v}" with "${fix}" for property \`${name}\``,
          );
          return fix;
        }
        return v;
      })
      .toSorted();

    if (valuesList[0]!.startsWith("Ofx")) {
      items.push(
        `make_enum_from_paths!(${name}, ${
          valuesList.map((v) => {
            const path = `crate::sys_umbrella::k${v}`;
            return `#[doc = "See: [\`${path}\`]."] ${v} => ${path}`;
          }).join(", ")
        });`,
      );
    } else {
      items.push(
        `make_enum_from_idents!(${name}, ${
          valuesList.map((v) => `r#${v} : c"${v}"`).join(", ")
        });`,
      );
    }
  }

  return items.join("\n");
}

await main(doParseArgs(Deno.args) as Args);
