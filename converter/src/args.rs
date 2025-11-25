#[derive(Debug, clap::Parser)]
pub struct Args {
    /// Input file path
    #[arg(short = 'i', long)]
    pub input: std::path::PathBuf,

    #[arg(short = 'f', long, value_enum)]
    pub input_format: InputFormat,
}

#[derive(Debug, clap::ValueEnum, Clone)]
pub enum InputFormat {
    MT940,
}
