use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path of the ASM file
    pub file_path: String,

    /// Emit bin file as `out.bin`
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub emit_bin: bool,
    // #[command(subcommand)]
    // pub command: Option<Commands>,
}

// #[derive(Subcommand)]
// pub enum Commands {

// }

pub fn get_cli() -> Cli {
    Cli::parse()
}
