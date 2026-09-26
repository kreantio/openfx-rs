vendor-c-path := "vendor/openfx/include"
data-generated-path := "data/generated"

generate-data:
    rm -rf "{{ data-generated-path }}"
    mkdir -p "{{ data-generated-path }}"
    cargo run --release --package openfx-datagen --bin cli -- \
        --input-c-headers "{{ vendor-c-path }}" \
        --output-data "{{ data-generated-path }}"

test-data:
    cd data && just test
