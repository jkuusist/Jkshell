pub use std::env;
pub use std::path::Path;

pub fn change_directory(args: String) {
	let result;

	if args.is_empty() {
		let home_path = env::var("HOME").expect("could not open HOME");
		result = env::set_current_dir(Path::new(&home_path));
	} else {
		let path = args.split_whitespace().next().unwrap();
		result = env::set_current_dir(Path::new(path));
	}

	match result {
		Ok(_) => return,
		Err(error) => println!("{}", error),
	}
}

pub fn echo_builtin(args: &str) {
	let output = &args[1..args.len()];

	println!("{}", output);
}
