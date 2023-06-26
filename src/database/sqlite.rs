use rusqlite::Connection;

use crate::commons::exit_error;

use super::{models::ToQuery, DataBase};

pub struct Sqlite {
	pub connection: Connection,
}

impl Sqlite {
	pub fn new(name: String) -> Self {
		let connection =
			Connection::open(&name).expect(&format!("Error trying to open the database: {name}"));

		let sql = Sqlite { connection };
		sql.create_schema();

		sql
	}

	fn create_schema(&self) {
		self
			.connection
			.execute("PRAGMA foreign_keys = ON", [])
			.unwrap();
		self.connection.execute("
            CREATE TABLE IF NOT EXISTS T_DOCUMENT (id INTEGER PRIMARY KEY AUTOINCREMENT, path TEXT UNIQUE, total_terms INTEGER)"
            , []).unwrap();
		self.connection.execute(
            "CREATE TABLE IF NOT EXISTS T_WORD (id INTEGER PRIMARY KEY AUTOINCREMENT, word TEXT UNIQUE, apparitions INTEGER)"
            , []).unwrap();
		self.connection.execute(
            "CREATE TABLE IF NOT EXISTS T_DOC_WORD (id_doc INTEGER NOT NULL, id_word INTEGER NOT NULL, apparition INTEGER, tf INTEGER, FOREIGN KEY (id_doc) REFERENCES T_DOCUMENT (id), FOREIGN KEY (id_word) REFERENCES T_WORD (id))"
            , []).unwrap();
	}
}

impl DataBase for Sqlite {
	fn save<T: ToQuery>(&self, obj: T) -> usize {
		let q = obj.to_query();
		println!("query: {q}");

		self.connection.execute(q.as_str(), []).unwrap();

		self.connection.last_insert_rowid() as usize
	}

	fn save_all<T: ToQuery>(&self, objs: &[T], table: String, extent_query: String) {
		if objs.is_empty() {
			exit_error("Trying to save empty array");
		}

		let items: Vec<String> = objs.iter().map(|o| o.to_mult_query()).collect();

		let q = format!(
			"INSERT INTO {table} {} VALUES {} {extent_query};",
			objs[0].fields(),
			items.join(",")
		);

		self.connection.execute(q.as_str(), []).unwrap();
	}

	fn save_all_doc_word(&self, objs: &Vec<super::models::doc_word::DbDocWordToSave>) {
		let values: Vec<String> = objs
			.iter()
			.map(|o| {
				format!(
					"({}, (SELECT id FROM T_WORD tw WHERE tw.word = \"{}\"), {}, {})",
					o.1, o.0, o.2, o.3
				)
			})
			.collect();

		let q = format!(
			"INSERT INTO T_DOC_WORD (id_doc, id_word, apparition, tf)
            VALUES {}",
			values.join(",")
		);

		self.connection.execute(&q, []).unwrap();
	}
}
