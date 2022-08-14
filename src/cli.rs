#[derive(clap::Parser)]
pub struct Cli {
	#[clap(subcommand)]
	pub action: CliAction,
} 

#[derive(clap::Subcommand)]
pub enum CliAction {
	Migrate {
		db_file: String,
		dir: String,
	},
}
