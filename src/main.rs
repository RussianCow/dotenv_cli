mod parser;

use std::env;
use std::path::PathBuf;
use std::process::Command;

use parser::parse_file;

fn main() {
	let cwd =
		env::current_dir().expect("Could not get current working directory.");
	let path: PathBuf = [cwd, ".env".into()].iter().collect();
	let additional_env_vars = parse_file(&path);

	let args: Vec<String> = env::args().collect();
	let command_name = &args.get(1)
		.expect("You must pass a command to run as an argument.");
	let command_args = &args[2..];

	let mut command = Command::new(command_name);
	command.args(command_args);
	for (key, value) in additional_env_vars.iter() {
		command.env(key, value);
	}
	command.spawn().expect(&format!(
		"Could not run the command: {}",
		command_name
	));
}
