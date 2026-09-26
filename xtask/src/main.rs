use clap::Parser;

mod cmd_jsonschema;

#[derive(clap::Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Jsonschema(cmd_jsonschema::Jsonschema),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Jsonschema(jsonschema) => cmd_jsonschema::run(jsonschema)?,
    }

    Ok(())
}
