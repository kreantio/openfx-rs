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
    output_code_low: PathBuf,

    #[arg(long)]
    output_code_low_plugin: PathBuf,
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let codegen_config_str = std::fs::read_to_string(&args.codegen_config)?;
    let codegen_config = openfx_codegen::config::CodegenConfig::from_yaml_str(&codegen_config_str)?;

    let input_bindings =
        openfx_codegen::input_bindings_data::load_input_bindings_data(args.input_bindings_data)?;
    let info = openfx_codegen::input_bindings_data::collect_info(&input_bindings);

    let input_metadata = openfx_codegen::input_metadata::load_input_metadata(args.input_metadata)?;

    std::fs::create_dir_all(&args.output_code_low)?;
    std::fs::create_dir_all(&args.output_code_low_plugin)?;

    {
        let p = args.output_code_low.join("c_statuses.rs");
        openfx_codegen::ex_codegen_c::gen_low_statuses(&codegen_config, &p, info.statuses)?;
    }
    {
        let p = args.output_code_low.join("c_enums.rs");
        openfx_codegen::ex_codegen_c::gen_low_enums_from_c(&p, info.c_enums)?;
    }

    {
        let p = args.output_code_low_plugin.join("c_suites");
        std::fs::create_dir_all(&p)?;
        openfx_codegen::ex_codegen_c::gen_low_plugin_suites(&codegen_config, &p, info.suites)?;
    }
    {
        let p = args.output_code_low_plugin.join("c_objects");
        std::fs::create_dir_all(&p)?;
        openfx_codegen::ex_codegen_c::gen_low_plugin_objects(
            &codegen_config,
            &p,
            info.direct_handle_usages_in_suite_functions,
        )?;
    }

    {
        let p = args.output_code_low.join("meta_enums.rs");
        openfx_codegen::ex_codegen_metadata::gen_low_enums_from_metadata(&p, &input_metadata)?;
    }

    {
        let p = args.output_code_low_plugin.join("meta_actions.rs");
        openfx_codegen::ex_codegen_metadata::gen_low_plugin_actions(
            &codegen_config,
            &p,
            &input_metadata,
        )?;
    }

    {
        let p = args.output_code_low_plugin.join("meta_property_sets.rs");
        openfx_codegen::ex_codegen_metadata::gen_low_plugin_property_sets(
            &codegen_config,
            &p,
            &input_metadata,
        )?;
    }

    Ok(())
}
