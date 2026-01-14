use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path of the ASM file
    pub file_path: String,

    /// Emit bin file as `out.bin`
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub emit_binary: bool,

    /// Indicates that the path given is a binary file (UNIMPLEMENTED)
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub binary_file: bool,
}

pub fn get_cli() -> Cli {
    Cli::parse()
}
