#!/usr/bin/env sh

set -e

ARG_TARGET_TRIPLE=""
ARG_BASE_NAME=""
ARG_INFO_PLIST=""

while [ $# -gt 0 ]; do
  case "$1" in
    -h|--help)
      echo "Usage: $0 [--target-triple <target-triple>] --base-name <base-name> --info-plist <info-plist-path>"
      exit 0
      ;;
    --target-triple)
      ARG_TARGET_TRIPLE="$2"
      shift 2
      ;;
    --base-name)
      ARG_BASE_NAME="$2"
      shift 2
      ;;
    --info-plist)
      ARG_INFO_PLIST="$2"
      shift 2
      ;;
    *)
      echo "Error: Unknown option: $1" >&2
      exit 1
      ;;
  esac
done

if [ -z "$ARG_TARGET_TRIPLE" ]; then
  ARG_TARGET_TRIPLE=$(rustc -vV | sed -n 's/host: //p')
fi

if [ -z "$ARG_BASE_NAME" ]; then
  echo "Error: Missing option: \`--base-name\`" >&2
  exit 1
fi

if [ -z "$ARG_INFO_PLIST" ]; then
  echo "Error: Missing option: \`--info-plist\`" >&2
  exit 1
fi

main() {
  echo ARG_TARGET_TRIPLE="$ARG_TARGET_TRIPLE"
  echo ARG_BASE_NAME="$ARG_BASE_NAME"
  echo ARG_INFO_PLIST="$ARG_INFO_PLIST"

  TARGET_INFO=$(get_target_info "$ARG_TARGET_TRIPLE")
  IFS=' ' read -r OFX_ARCHITECTURE LIBRARY_EXTENSION LIBRARY_PREFIX <<< "$TARGET_INFO"
  echo OFX_ARCHITECTURE="$OFX_ARCHITECTURE"
  echo LIBRARY_EXTENSION="$LIBRARY_EXTENSION"
  echo LIBRARY_PREFIX="$LIBRARY_PREFIX"

  CRATE_MANIFEST_DIR=$(dirname "$(cargo locate-project --message-format plain)")
  if [ -z "$CRATE_MANIFEST_DIR" ]; then
    echo "Error: \$CRATE_MANIFEST_DIR is empty!" >&2
    exit 1
  fi
  echo CRATE_MANIFEST_DIR="$CRATE_MANIFEST_DIR"

  rm -rf "$CRATE_MANIFEST_DIR/artifact"
  cargo +nightly build -Z unstable-options --release --target "$ARG_TARGET_TRIPLE" --artifact-dir artifact

  set -- $CRATE_MANIFEST_DIR/artifact/*
  case $# in
    0)
      echo "Error: No artifacts were found in: $CRATE_MANIFEST_DIR/artifact" >&2
      exit 1
      ;;
    1)
      ARTIFACT_BINARY_PATH="$1"
      ;;
    *)
      echo "Error: Multiple artifacts are not supported. They were found in: $CRATE_MANIFEST_DIR/artifact" >&2
      exit 1
      ;;
  esac
  echo ARTIFACT_BINARY_PATH="$ARTIFACT_BINARY_PATH"

  BUILD_ARCHITECTURE_PATH="$CRATE_MANIFEST_DIR/build/$OFX_ARCHITECTURE"
  rm -rf "$BUILD_ARCHITECTURE_PATH"

  BUILD_BUNDLE_CONTENTS_PATH="$BUILD_ARCHITECTURE_PATH/$ARG_BASE_NAME.ofx.bundle/Contents"
  mkdir -p "$BUILD_BUNDLE_CONTENTS_PATH/Resources" "$BUILD_BUNDLE_CONTENTS_PATH/$OFX_ARCHITECTURE"

  cp "$ARG_INFO_PLIST" "$BUILD_BUNDLE_CONTENTS_PATH/Resources"
  cp "$ARTIFACT_BINARY_PATH" "$BUILD_BUNDLE_CONTENTS_PATH/$OFX_ARCHITECTURE/$ARG_BASE_NAME.ofx"
}

get_target_info() {
  local TARGET_TRIPLE=$1
  local OFX_ARCHITECTURE=""
  local LIBRARY_EXTENSION=""
  local LIBRARY_PREFIX=""

  # took from: https://github.com/ntsc-rs/ntsc-rs/blob/af9833b4bb81f195f7fe4a3667211f2a94139a42/xtask/src/util/targets.rs
  #
  # license: MIT (Copyright © valadaptive)
  # license link: https://github.com/ntsc-rs/ntsc-rs/blob/af9833b4bb81f195f7fe4a3667211f2a94139a42/LICENSE-MIT
  case "$TARGET_TRIPLE" in
    "x86_64-unknown-linux-gnu")
      OFX_ARCHITECTURE="Linux-x86-64"
      LIBRARY_EXTENSION="so"
      LIBRARY_PREFIX="lib"
      ;;
    "i686-unknown-linux-gnu")
      OFX_ARCHITECTURE="Linux-x86"
      LIBRARY_EXTENSION="so"
      LIBRARY_PREFIX="lib"
      ;;
    "x86_64-pc-windows-msvc")
      OFX_ARCHITECTURE="Win64"
      LIBRARY_EXTENSION="dll"
      LIBRARY_PREFIX=""
      ;;
    "i686-pc-windows-msvc")
      OFX_ARCHITECTURE="Win32"
      LIBRARY_EXTENSION="dll"
      LIBRARY_PREFIX=""
      ;;
    "x86_64-apple-darwin")
      OFX_ARCHITECTURE="MacOS"
      LIBRARY_EXTENSION="dylib"
      LIBRARY_PREFIX="lib"
      ;;
    "aarch64-apple-darwin")
      OFX_ARCHITECTURE="MacOS"
      LIBRARY_EXTENSION="dylib"
      LIBRARY_PREFIX="lib"
      ;;
    *)
      echo "Error: Unsupported target triple: $TARGET_TRIPLE" >&2
      exit 1
      ;;
  esac

  echo "$OFX_ARCHITECTURE $LIBRARY_EXTENSION $LIBRARY_PREFIX"
}

main
