mod util;
mod mcl;
mod obj;
mod convert;
mod cli;

use clap::Parser;
// use clap::Parser;
use cli::{Cli, Commands};

fn main() {
	let cli = Cli::parse();

	let response = match &cli.command {
		Commands::Convert { input, output } => {
			convert::main(input, output)
		}
	};

	std::process::exit(match response {
		Ok(()) => 0,
		Err(e) => {
			println!("Error: {}", e);
			1
		}
	});
}