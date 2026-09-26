#!/usr/bin/env sh

set -euo pipefail

for file in generated/bindings/*; do
    cargo xtask jsonschema --schema generated/schemata/bindings.schema.json --input "$file" &
done

wait