publish: generate-all
    cargo publish -p openfx-internal-macros
    cargo publish -p openfx

generate-all: generate-for-crate-openfx

generate-for-crate-openfx:
    cd crates/openfx && just generate-all

detect-stale-generated-contents: generate-all
    #!/usr/bin/env sh
    if ! git diff --quiet; then
        echo "Stale generated contents detected."
        echo "Please run \`just generate-all\` and commit the changes."
        echo "stale files:"
        git --no-pager diff --name-only
        exit 1
    fi
