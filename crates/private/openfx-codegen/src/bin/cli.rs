use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    codegen_config: PathBuf,

    #[arg(long)]
    input_data: PathBuf,

    #[arg(long)]
    output: PathBuf,

    #[arg(long)]
    output_intermediate: PathBuf,
}

pub fn main() {
    let args = Args::parse();

    let codegen_config_str =
        std::fs::read_to_string(&args.codegen_config).expect("Failed to read codegen config file");
    let codegen_config: openfx_codegen::CodegenConfig =
        openfx_codegen::CodegenConfig::from_toml_str(&codegen_config_str)
            .expect("Failed to parse codegen config file");

    let input_data = openfx_codegen::input_data::load_input_data(args.input_data)
        .expect("Failed to load input data");

    let output_folder_bindings = args.output.join("c_bindings");
    std::fs::create_dir_all(&output_folder_bindings).expect("Failed to create folder `c_bindings`");
    openfx_codegen::bindgen::generate_bindings(&input_data, &output_folder_bindings)
        .expect("Failed to generate bindings");

    let info = openfx_codegen::input_data::collect_info(&input_data);

    let output_folder_c = args.output.join("code_from_c");
    std::fs::create_dir_all(&output_folder_c).expect("Failed to create folder `code_from_c`");
    openfx_codegen::ex_codegen::gen_low_statuses(
        &codegen_config,
        &output_folder_c.join("low_statuses.rs"),
        info.statuses,
    )
    .expect("Failed to execute `gen_low_statuses`");
    openfx_codegen::ex_codegen::gen_low_enums_from_c(
        &output_folder_c.join("low_enums_from_c.rs"),
        info.c_enums,
    )
    .expect("Failed to execute `gen_low_enums_from_c`");
    openfx_codegen::ex_codegen::gen_low_plugin_suites(
        &codegen_config,
        &output_folder_c,
        info.suites,
    )
    .expect("Failed to execute `gen_low_plugin_suites`");
    openfx_codegen::ex_codegen::gen_low_plugin_objects(
        &codegen_config,
        &output_folder_c.join("low_objects_plugin.rs"),
        info.direct_handle_usages_in_suite_functions,
    )
    .expect("Failed to execute `gen_low_plugin_objects`");
    openfx_codegen::ex_codegen::gen_data_root_idents(
        &args
            .output_intermediate
            .join("root_item_idents_per_header.json"),
        info.root_item_idents_per_header,
    )
    .expect("Failed to execute `gen_data_root_idents`");
}
