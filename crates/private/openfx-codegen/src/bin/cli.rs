use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    codegen_config: PathBuf,

    #[arg(long)]
    input_bindings_data: PathBuf,

    #[arg(long)]
    input_metadata: PathBuf,

    #[arg(long)]
    output: PathBuf,

    #[arg(long)]
    output_intermediate: PathBuf,
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let codegen_config_str = std::fs::read_to_string(&args.codegen_config)?;
    let codegen_config = openfx_codegen::config::CodegenConfig::from_toml_str(&codegen_config_str)?;

    let input_bindings =
        openfx_codegen::input_bindings_data::load_input_bindings_data(args.input_bindings_data)?;
    let info = openfx_codegen::input_bindings_data::collect_info(&input_bindings);

    let input_metadata = openfx_codegen::input_metadata::load_input_metadata(args.input_metadata)?;

    let output_folder_c = args.output.join("code_from_c");
    std::fs::create_dir_all(&output_folder_c)?;

    let output_folder_metadata = args.output.join("code_from_metadata");
    std::fs::create_dir_all(&output_folder_metadata)?;

    openfx_codegen::ex_codegen_c::gen_low_statuses(
        &codegen_config,
        &output_folder_c.join("low_statuses.rs"),
        info.statuses,
    )?;
    openfx_codegen::ex_codegen_c::gen_low_enums_from_c(
        &output_folder_c.join("low_enums_from_c.rs"),
        info.c_enums,
    )?;
    openfx_codegen::ex_codegen_c::gen_low_plugin_suites(
        &codegen_config,
        &output_folder_c,
        info.suites,
    )?;
    openfx_codegen::ex_codegen_c::gen_low_plugin_objects(
        &codegen_config,
        &output_folder_c.join("low_objects_plugin.rs"),
        info.direct_handle_usages_in_suite_functions,
    )?;
    openfx_codegen::ex_codegen_c::gen_data_root_idents(
        &args
            .output_intermediate
            .join("root_item_idents_per_header.json"),
        info.root_item_idents_per_header,
    )?;

    openfx_codegen::ex_codegen_metadata::gen_low_enums_from_metadata(
        &output_folder_metadata.join("low_enums.rs"),
        &input_metadata,
    )?;

    Ok(())
}
