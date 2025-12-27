use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path of the ASM file
    pub file_path: String,

    // #[command(subcommand)]
    // pub command: Option<Commands>,
}

// #[derive(Subcommand)]
// pub enum Commands {

// }

pub fn get_cli() -> Cli {
    Cli::parse()
}
