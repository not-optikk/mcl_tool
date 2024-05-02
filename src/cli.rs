use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
	Convert {
		input: std::path::PathBuf,
		
		#[arg(short, long)]
		output: std::path::PathBuf
	}
}