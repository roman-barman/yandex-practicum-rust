#[derive(Debug, clap::Parser)]
pub struct Args {
    /// Input file path
    #[arg(short = 'i', long)]
    pub input: std::path::PathBuf,

    /// Input file format
    #[arg(short = 'f', long, value_enum)]
    pub input_format: Format,

    /// Output file format
    #[arg(short = 'o', long, value_enum)]
    pub output_format: Option<Format>,
}

#[derive(Debug, clap::ValueEnum, Clone)]
pub enum Format {
    MT940,
    CAMT053,
}
