use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub fn parse_file(path: &Path) -> HashMap<String, String> {
	let file = File::open(path).expect("There is no .env file in the current directory.");
	let reader = BufReader::new(file);
	let mut vars: HashMap<String, String> = HashMap::new();

	for line in reader.lines() {
		let maybe_key_value = parse_line(line.unwrap());
		if maybe_key_value.is_none() {
			continue;
		}
		let (key, value) = maybe_key_value.unwrap();
		vars.insert(key, value);
	}

	vars
}

fn parse_line(line: String) -> Option<(String, String)> {
	let mut key = String::new();
	let mut value = String::new();
	let mut is_past_equal_sign = false;

	for character in line.chars() {
		let is_first = key.is_empty();

		// Skip comments
		if is_first && character == '#' {
			break;
		}

		if !character.is_ascii() {
			panic!("Non-ASCII character found: {}", character);
		}
		if is_first && character.is_numeric() {
			panic!("The first character of an environment variable cannot be a number.");
		}
		if !is_past_equal_sign
			&& (!character.is_numeric() && !character.is_uppercase() && character != '=')
		{
			panic!("Invalid character in key name: {}", character);
		}

		if is_past_equal_sign {
			value = value + &character.to_string();
		} else {
			if character == '=' {
				is_past_equal_sign = true;
			} else {
				key = key + &character.to_string();
			}
		}
	}

	if key.is_empty() {
		None
	} else {
		Some((key, value))
	}
}
