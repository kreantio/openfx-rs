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
}

pub fn main() {
    let args = Args::parse();

    openfx_bindgen::bindings_for_c_headers::generate_bindings_for_c_headers(
        args.input_c_headers,
        args.output_c_bindings,
        args.output_code_from_c,
    );
}
