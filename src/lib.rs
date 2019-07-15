//Joseph Harrison 2019
use std::fs;
use std::str;

pub struct Config {

	pub filename: String,
	pub function: Mode,
}

pub enum Mode {

	Encipher(u8),
	Decipher(u8),
}

impl Config {

	pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {

		//make sure we have 3 command line args
		if args.len() < 4 {

			return Err("insufficient args");
		}
		let filename = args[1].clone();
		
		//cast shift arg as int
		let shift = match args[2].parse() {

			Ok(value) => value,
			Err(_) => {
				return Err("non-integer shift arg");
			},
		};
		//create function from switches
		let function = match &args[3][..] {

			"-e" => Mode::Encipher(shift),
			"-d" => Mode::Decipher(shift),
			_ => {
				return Err("unknown switch");
			},
		};

		Ok(Config { filename, function })
	}
}

pub fn run(config: Config) -> Result<(), &'static str> {

	let mut contents = match fs::read_to_string(config.filename) {

		Ok(value) => value,
		Err(_) => {
			return Err("error opening file")
		},
	};
	contents = contents.replace("\n", " ");

	let ciphered_bytes = cipher(&contents, &config.function);

	let contents = match str::from_utf8(&ciphered_bytes) {

		Ok(value) => value,
		Err(_) => {
			return Err("error transforming text")
		},
	};

	println!("{}", contents);

	Ok(())
}

pub fn cipher<'a>(contents: &'a str, function: &Mode) -> Vec<u8> {

	let contents: Vec<u8> = match function {

		Mode::Encipher(shift) => {

			contents.as_bytes().iter()
			.map(|c| (c + shift) % 255)
			.collect()
		},
		Mode::Decipher(shift) => {

			contents.as_bytes().iter()
			.map(|c| (c - shift) % 255)
			.collect()
		},
	};
	contents
}





