mod cli;
mod migrate;

use clap::Parser;
use migrate::migrate;

fn main() {
	let args = cli::Cli::parse();
	let res = match args.action {
		cli::CliAction::Migrate { db_file, dir } => {
			migrate(&db_file, &dir)
		}
	};

	match res {
		Ok(_) => {}
		Err(e) => {
			eprintln!("Error: {}", e);
			std::process::exit(1);
		}
	}
}
