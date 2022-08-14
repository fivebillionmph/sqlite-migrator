use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::io::Write;
use anyhow::Result as Res;
use jgtbrs::error::AppError;
use regex::Regex;

pub fn migrate(db_file: &str, dir: &str) -> Res<()> {
	let current_version = if Path::new(db_file).exists() {
		get_current_version(db_file)?
	} else {
		0
	};

	for f in get_migration_files(dir, current_version)? {
		f.migrate(db_file)?;
	}

	Ok(())
}

struct MigrationFile {
	full_path: PathBuf,
	version: u32,
}
impl MigrationFile {
	fn new(p: &Path, version: u32) -> Res<Self> {
		Ok(Self {
			full_path: fs::canonicalize(p)?,
			version,
		})
	}

	fn migrate(&self, db_file: &str) -> Res<()> {
		let mut cmd = Command::new("sqlite3")
			.arg(db_file)
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.spawn()?;

		let stdin = cmd.stdin.as_mut().ok_or_else(|| AppError::new("Could not pipe in command to sqlite3"))?;
		stdin.write_all(fs::read(&self.full_path)?.as_slice())?;
		drop(stdin);
		if !cmd.wait()?.success() {
			return Err(AppError::new(&format!("Sqlite errored when migrating file: {}", jgtbrs::util::path_to_string(&self.full_path.as_path())?)).into());
		}

		let mut echo_cmd = Command::new("echo")
			.arg(&format!("PRAGMA user_version={};", self.version))
			.stdout(Stdio::piped())
			.spawn()?;

		let mut sqlite_cmd = Command::new("sqlite3")
			.arg(db_file)
			.stdin(echo_cmd.stdout.take().ok_or_else(|| AppError::new("Could not pipe to sqlite command."))?)
			.stdout(Stdio::piped())
			.spawn()?;
		echo_cmd.wait()?;
		if !sqlite_cmd.wait()?.success() {
			return Err(AppError::new(&format!("Sqlite errored when update version to: {}", self.version)).into());
		}

		Ok(())
	}
}

fn get_current_version(db_file: &str) -> Res<u32> {
	let mut echo_cmd = Command::new("echo")
		.arg("PRAGMA user_version;")
		.stdout(Stdio::piped())
		.spawn()?;

	let sqlite_cmd = Command::new("sqlite3")
		.arg(db_file)
		.stdin(echo_cmd.stdout.take().ok_or_else(|| AppError::new("Could not pipe to sqlite command."))?)
		.stdout(Stdio::piped())
		.spawn()?;
	echo_cmd.wait()?;

	let output = sqlite_cmd.wait_with_output()?;
	let sqlite_stdout = std::str::from_utf8(output.stdout.as_slice())?.trim();
	let version: u32 = sqlite_stdout.parse()?;
	Ok(version)
}

fn get_migration_files(dir: &str, current_version: u32) -> Res<Vec<MigrationFile>> {
	let mut res = Vec::new();
	let mut migration_versions = HashSet::new();
	for file in std::fs::read_dir(dir)? {
		let file = file?;
		if !file.path().is_file() {
			continue;
		}
		let file_name = jgtbrs::util::path_to_string(&file.path())?;

		let re = Regex::new(r"m_(\d+).sql$")?;

		if let Some(cap) = re.captures(&file_name) {
			let version_str = cap.get(1).ok_or_else(|| AppError::new("Could not read version from filename."))?.as_str();
			let version_number: u32 = version_str.parse()?;
			if version_number <= current_version {
				continue;
			}
			if migration_versions.contains(&version_number) {
				return Err(AppError::new(&format!("Migration version repeated in directory: {}", version_number)).into());
			}
			migration_versions.insert(version_number);

			res.push(MigrationFile::new(&file.path(), version_number)?);
		}
	}

	res.sort_by_key(|a| a.version);

	Ok(res)
}
