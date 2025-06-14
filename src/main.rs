use clap::{Parser,Subcommand,Args};

use crate::chunk_type::ChunkType;
use crate::chunk::Chunk;
use crate::png::Png;
pub mod chunk_type;
pub mod chunk;
pub mod png;
use std::convert::TryFrom;
use std::fs;
use std::path::{PathBuf};

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
    file_path: PathBuf,
    ctype: ChunkType,
    message: String,
    #[arg(short, long)]
    output_file: Option<PathBuf>,
}
#[derive(Args)]
struct  DecodeArgs {
    file_path: PathBuf,
    ctype: ChunkType,
}
#[derive(Args)]
struct  RemoveArgs {
    file_path: PathBuf,
    ctype: ChunkType,
}
#[derive(Args)]
struct  PrintArgs {
    file_path: PathBuf,
}
fn encode(args: EncodeArgs) -> crate::Result<()> {
    let output_file = args.output_file.unwrap_or_else(||  args.file_path.clone());
    let png_bytes = fs::read(args.file_path)?;
    let mut png: Png = TryFrom::try_from(png_bytes.as_slice())?;
    let chunk = Chunk::new(args.ctype, args.message.as_bytes().to_vec());
    png.append_chunk(chunk);
    fs::write(output_file, png.as_bytes())?;
    Ok(())
}
fn decode(args: DecodeArgs) -> crate::Result<()> {
    let png_bytes = fs::read(args.file_path)?;
    let png: Png = TryFrom::try_from(png_bytes.as_slice())?;
    let chunk = png.chunk_by_type(&args.ctype.to_string());
    if let Some(c) = chunk {
        println!("{}", c);
    }
    Ok(())
}
fn remove(args: RemoveArgs) -> crate::Result<()> {
    let png_bytes = fs::read(&args.file_path)?;
    let mut png: Png = TryFrom::try_from(png_bytes.as_slice())?;
    match png.remove_first_chunk(&args.ctype.to_string()) {
        Ok(chunk) => {
            fs::write(args.file_path, png.as_bytes())?;
            println!("Removed chunk {}", chunk);
        }
        Err(e) => return Err(Box::new(e)),
    }
    Ok(())
}
fn print(args: PrintArgs) -> crate::Result<()> {
    let png_bytes = fs::read(args.file_path)?;
    let png: Png = TryFrom::try_from(png_bytes.as_slice())?;
    println!("{}", png);
    Ok(())
}
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
fn main() -> Result<()>{
    let cli = Cli::parse();
    match cli.command {
        Commands::Encode(args) => encode(args),
        Commands::Decode(args) => decode(args),
        Commands::Remove(args) => remove(args),
        Commands::Print(args) => print(args),
    }?;
    Ok(())
}