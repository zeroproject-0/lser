use pdf_extract::extract_text;
use std::collections::{HashMap, VecDeque};
use std::fs::{read_dir, read_to_string, ReadDir};
use std::path::PathBuf;

use crate::commons::exit_error;

pub struct Document {
	pub count_words: i32,
	pub path: PathBuf,
	pub term_freq: HashMap<String, (i32, f32)>,
}

impl Document {
	fn populate_term_freq(&mut self, text: &str) {
		let mut words: Vec<String> = text
			.replace('\n', " ")
			.split(' ')
			.map(str::to_owned)
			.collect();

		for word in &mut words {
			if word.is_empty() {
				continue;
			}

			match self.term_freq.get_mut(word) {
				Some(value) => value.0 += 1,
				None => {
					self.term_freq.insert(word.to_owned(), (1, 0.0));
				}
			};

			self.count_words += 1;
		}
	}

	pub fn new(text: &str, path: PathBuf) -> Self {
		let mut doc = Document {
			count_words: 0,
			path,
			term_freq: HashMap::new(),
		};

		doc.populate_term_freq(text);

		let freq = doc.term_freq.clone();

		for (key, value) in freq.iter() {
			doc.term_freq.insert(
				key.clone(),
				(value.0, (value.0 as f32) / (doc.count_words as f32)),
			);
		}

		doc
	}
}

pub fn index_from_str(text: &str, path: &PathBuf) -> Option<Document> {
	let doc = Document::new(text, path.to_path_buf());

	if doc.count_words == 0 {
		return None;
	}

	Some(doc)
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

// idf -> Invesr Document Frequency
pub fn calc_idf(term: &str, docs: Vec<Document>) -> f32 {
	let total_documents = docs.len() as f32;

	let mut appears = 1;

	for doc in docs {
		if doc.term_freq.contains_key(term) {
			appears += 1;
		}
	}

	(total_documents / (appears as f32)).log10()
}

pub fn extract(dir: ReadDir) -> Vec<Document> {
	let mut docs: Vec<Document> = Vec::new();

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

				let doc: Option<Document> = match content {
					Some(content) => index_from_str(&content, &item),
					None => {
						println!("Can't get content from: {}", item.display());
						None
					}
				};

				if let Some(doc) = doc {
					docs.push(doc);
				}
			};
		}
	}

	return docs;
}

pub fn index(mut args: VecDeque<String>) {
	let path = args.pop_front().unwrap_or(String::from("files"));

	match read_dir(path) {
		Ok(items) => {
			for doc in extract(items) {
				let mut tf_sorted: Vec<(String, (i32, f32))> = doc.term_freq.into_iter().collect();
				tf_sorted.sort_by(|(_, a), (_, b)| b.1.total_cmp(&a.1));

				println!("{}", doc.path.display());

				for (term, (freq, tf)) in tf_sorted.iter().take(30) {
					println!("\t{term}: {freq}, {tf}");
				}
			}
		}
		Err(_) => exit_error("The argument must be a directory"),
	}
}
