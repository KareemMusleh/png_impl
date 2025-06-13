use clap::{Parser,Subcommand,Args};

use crate::chunk_type::ChunkType;
use crate::png::Png;
pub mod chunk_type;
pub mod chunk;
pub mod png;
use std::convert::TryFrom;
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}
#[derive(Subcommand)]
enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}
#[derive(Args)]
struct EncodeArgs {
    file_path: String,
    ctype: String,
    message: String,
    #[arg(short, long)]
    output_file: Option<String>,
}
#[derive(Args)]
struct  DecodeArgs {
    file_path: String,
    ctype: String,
}
#[derive(Args)]
struct  RemoveArgs {
    file_path: String,
    ctype: String,
}
#[derive(Args)]
struct  PrintArgs {
    file_path: String,
}
fn encodeMsg(file_path: &str, ctype: &str, message: &str, output_file: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let output_file = output_file.unwrap_or_else(||  file_path);
    let png_bytes = fs::read(file_path)?;
    let png: Png = TryFrom::try_from(png_bytes.as_slice())?;
    let chunk = png.chunk_by_type(ctype);
    // chunk.
    Ok(())
}
fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Encode(args) => {
            encodeMsg(
                &args.file_path,
                &args.ctype,
                &args.message,
                args.output_file.as_deref()
            );
        }
        Commands::Decode(args) => {
            decodeMsg(
                &args.file_path,
                &args.ctype,
                &args.message,
                args.output_file.as_deref()
            );
        }
        Commands::Remove(args) => {
            removeChunk(
                &args.file_path,
                &args.ctype,
            );
        }
        Commands::Print(args) => {
            printChunk(
                &args.file_path,
            );
        }
    }
}