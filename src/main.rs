use clap::Parser;
use string_dataframe::read_parquet::read_parquet;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Name of the person to greet
    file_path: String,

    /// Number of times to greet
    #[clap(arg_enum, default_value = "cli", short, long)]
    output_type: OutputType,
}

#[derive(clap::ArgEnum, Clone, Debug)]
enum OutputType {
    Cli,
    Csv,
}

fn main() {
    let args = Args::parse();
    let dataframe = read_parquet(&args.file_path);
    match dataframe {
        Ok(dataframe) => match args.output_type {
            OutputType::Cli => {
                println!("{}", dataframe)
            }
            OutputType::Csv => {
                println!("{}", dataframe.to_csv())
            }
        },
        Err(e) => {
            eprintln!(
                "Could not load dataframe, the following error occured: {:?}",
                e
            )
        }
    }
}
