use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// the path to the input C headers directory
    #[arg(long)]
    input_c_headers: String,

    /// the path to the output directory for generated bindings
    #[arg(long)]
    output_c_bindings: String,

    #[arg(long)]
    output_code_from_c: String,

    #[arg(long)]
    output_intermediate: String,
}

pub fn main() {
    let args = Args::parse();

    openfx_codegen::bindings_for_c_headers::generate_bindings_for_c_headers(
        openfx_codegen::bindings_for_c_headers::Options {
            headers_folder: args.input_c_headers.into(),
            output_folder: args.output_c_bindings.into(),
            output_folder_c: args.output_code_from_c.into(),
            output_folder_intermediate: args.output_intermediate.into(),
        },
    );
}
