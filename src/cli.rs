#[derive(clap::Parser)]
/// Reads sqlite files in a directory and attempts to migrate them using the PRAGMA user_version to determine which files should be run in the database.
/// SQL files in the directory should follow the pattern m_{user_version}.sql
/// For example, the following file names will run and be executed in order: m_001.sql, m_2.sql, m_03.sql
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
