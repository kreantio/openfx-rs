#!/usr/bin/env sh

set -eu

for file in generated/bindings/*; do
    cargo xtask jsonschema --schema generated/schemata/bindings.schema.json --input "$file" &
done

wait