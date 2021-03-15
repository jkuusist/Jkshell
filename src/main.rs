use std::{env, fs};
use std::io::{self, Write, stdout, Error, ErrorKind};
use std::process::Command;

fn check_paths(input_buffer: &str) -> Result<String, io::Error> {
	let paths = env::var("PATH").expect("could not open PATH");

	for path in paths.split(":") {
		let program_str = format!("{}/{}", path, input_buffer);

		let metadata = fs::metadata(&program_str);

		if metadata.is_ok() {
			return Ok(program_str);
		}
	}

	Err(Error::new(ErrorKind::NotFound, "unknown command"))
}

fn main() {
	loop
	{
		print!("$>");	
		stdout().flush().expect("Could not flush output");

		let mut input_buffer = String::new();
		let stdin = io::stdin();

		stdin.read_line(&mut input_buffer).expect("Could not read input");

		input_buffer.truncate(input_buffer.len() - 1);

		let command_path = check_paths(&input_buffer);

		if command_path.is_ok() {
			let mut child = Command::new(input_buffer).spawn().expect("unknown command");

			child.wait().unwrap();
		} else {
			println!("unknown command");
		}
	}
}
