#[derive(Debug, clap::Parser)]
pub struct Args {
    /// Input file path
    #[arg(short = 'i', long)]
    pub input: std::path::PathBuf,

    /// Input file format
    #[arg(short = 'f', long, value_enum)]
    pub input_format: InputFormat,

    /// Output file format
    #[arg(short = 'o', long, value_enum)]
    pub output_format: Option<OutputFormat>,
}

#[derive(Debug, clap::ValueEnum, Clone)]
pub enum InputFormat {
    MT940,
}

#[derive(Debug, clap::ValueEnum, Clone)]
pub enum OutputFormat {
    MT940,
}
