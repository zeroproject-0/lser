use pdf_extract::extract_text;
use std::collections::VecDeque;
use std::fs::{read_dir, read_to_string, ReadDir};
use std::path::PathBuf;

use crate::commons::exit_error;

pub fn index_from_str(text: &str) {
	println!("{}", text);
}

pub fn is_valid_file(path: &PathBuf) -> bool {
	let files_types = vec!["pdf", "txt"];
	let file_extension = path.extension();

	match file_extension {
		Some(ext) => files_types.contains(&ext.to_str().unwrap()),
		_ => false,
	}
}

pub fn extract_pdf(path: &PathBuf) -> Option<String> {
	match extract_text(path) {
		Ok(text) => Some(text),
		Err(_) => None,
	}
}

pub fn extract_txt(path: &PathBuf) -> Option<String> {
	match read_to_string(path) {
		Ok(content) => Some(content),
		Err(_) => None,
	}
}

pub fn extract(dir: ReadDir) {
	for item in dir {
		let item = item.unwrap().path();

		if item.is_dir() {
			extract(read_dir(&item).unwrap());
		} else {
			if is_valid_file(&item) {
				let content = match item.extension().unwrap().to_str().unwrap() {
					"pdf" => extract_pdf(&item),
					"txt" => extract_txt(&item),
					_ => None,
				};

				match content {
					Some(content) => index_from_str(&content),
					None => println!("Can't get content from: {}", item.display()),
				}
			};
		}
	}
}

pub fn index(mut args: VecDeque<String>) {
	let path = args.pop_front().unwrap_or(String::from("files"));

	println!("{}", path);

	match read_dir(path) {
		Ok(items) => extract(items),
		Err(_) => exit_error("The argument must be a directory"),
	}
}
