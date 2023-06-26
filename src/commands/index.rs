use pdf_extract::extract_text;
use rusqlite::Connection;
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
			.to_uppercase()
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

	pub fn save_sqlite(self) {
		let conn = Connection::open("indexed.db").unwrap();
		conn.execute("PRAGMA foreign_keys = ON", []).unwrap();
		conn.execute("CREATE TABLE IF NOT EXISTS T_DOCUMENT (id INTEGER PRIMARY KEY AUTOINCREMENT, path TEXT, total_terms INTEGER)", []).unwrap();
		conn.execute("CREATE TABLE IF NOT EXISTS T_WORD (id INTEGER PRIMARY KEY AUTOINCREMENT, word TEXT UNIQUE, apparitions INTEGER)", []).unwrap();
		conn.execute("CREATE TABLE IF NOT EXISTS T_DOC_WORD (id_doc INTEGER NOT NULL, id_word INTEGER NOT NULL, apparition INTEGER, tf INTEGER, FOREIGN KEY (id_doc) REFERENCES T_DOCUMENT (id), FOREIGN KEY (id_word) REFERENCES T_WORD (id))", []).unwrap();

		let q_add_doc = "
				INSERT INTO T_DOCUMENT (path, total_terms) VALUES (?1, ?2);
		";

		conn
			.execute(
				q_add_doc,
				(self.path.display().to_string(), self.count_words),
			)
			.unwrap();

		let last_doc_id = conn.last_insert_rowid().to_string();

		let q_add_word = "
            INSERT INTO T_WORD (word, apparitions) 
            VALUES (?1, 1) 
            ON CONFLICT(word) DO UPDATE SET apparitions = apparitions + 1
		";

		let q_add_d_w = "
            INSERT INTO T_DOC_WORD (id_doc, id_word, apparition, tf)
            VALUES (?1, (SELECT id FROM T_WORD tw WHERE tw.word = ?2), ?3, ?4)
        ";

		for (word, (n_terms, tf)) in self.term_freq {
			conn.execute(q_add_word, [&word]).unwrap();
			conn
				.execute(q_add_d_w, (&last_doc_id, &word, n_terms, tf))
				.unwrap();
		}
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
				doc.save_sqlite();
			}
		}
		Err(_) => exit_error("The argument must be a directory"),
	}
}
