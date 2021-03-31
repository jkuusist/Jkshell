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

pub fn set_environment(args: &str) {
	let argv: Vec<&str> = args.split_whitespace().collect();

	if argv.len() == 0 {
		let mut env = Vec::new();

		for var in env::vars() {
			env.push(var);
		}

		env.sort_unstable();

		for (key, value) in env {
			println!("{}={}", key, value);
		}
	} else if argv.len() == 1 {
		let mut key = String::from(argv[0]);
		if let Some(eq_index) = key.find('=') {
			let mut value = &key.split_off(eq_index)[1..];
			if value.chars().nth(0).unwrap() == '\'' &&
			value.chars().nth(value.len() - 1).unwrap() == '\'' {
				value = &value[1..value.len() - 1];
			}

			env::set_var(key, value);

//			println!("key is: {}\nvalue is: {}", key, value);
		} else {
			println!("usage: export KEY='value'");
		}


//		env::set_var(argv[0], argv[1]);
	} else {
		println!("usage: export key value");
	}
}

pub fn unset_environment(args: &str) {
	let argv: Vec<&str> = args.split_whitespace().collect();

	if argv.len() == 1 {
		env::remove_var(argv[0]);
	}
}

pub fn echo_builtin(args: &str) {
	let output = &args[1..args.len()];

	println!("{}", output);
}
