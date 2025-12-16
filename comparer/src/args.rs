#[derive(Debug, clap::Parser)]
pub struct Args {
    /// Input file 1 path
    #[arg(long)]
    pub file1: std::path::PathBuf,

    /// Input file 1 format
    #[arg(long, value_enum)]
    pub file1_format: Format,

    /// Input file 2 path
    #[arg(long)]
    pub file2: std::path::PathBuf,

    /// Input file 2 format
    #[arg(long, value_enum)]
    pub file2_format: Format,
}

#[derive(Debug, clap::ValueEnum, Clone)]
pub enum Format {
    MT940,
    CAMT053,
}
