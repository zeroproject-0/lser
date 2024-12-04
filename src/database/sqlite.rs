use rusqlite::Connection;

use crate::commons::exit_error;

use super::{models::ToQuery, DataBase};

pub struct Sqlite {
	pub connection: Connection,
}

impl Sqlite {
	pub fn new(name: String) -> Self {
		let connection = Connection::open(&name)
			.unwrap_or_else(|_| panic!("Error trying to open the database: {name}"));

		let sql = Sqlite { connection };
		sql.create_schema();

		sql
	}

	fn create_schema(&self) {
		/*self
		.connection
		.execute("PRAGMA journal_mode = OFF", [])
		.unwrap();*/
		/*self
		.connection
		.execute("PRAGMA locking_mode = EXCLUSIVE", [])
		.unwrap();*/
		self
			.connection
			.execute("PRAGMA temp_store = MEMORY", [])
			.unwrap();
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

	fn save_all_doc_word(&mut self, objs: &Vec<super::models::doc_word::DbDocWordToSave>) {
		let batch_size = 100;
		let stmt = self.connection.transaction().unwrap();

		for i in 0..(objs.len() / batch_size) {
			let mut q = "INSERT INTO T_DOC_WORD VALUES ".to_owned();
			for j in 0..batch_size {
				let idx = j + (i * batch_size);
				let o = objs[idx].clone();
				q.push_str(&format!(
					"({}, (SELECT id FROM T_WORD tw WHERE tw.word LIKE \"{}%\" LIMIT 1), {}, {}),",
					o.id_doc, o.word, o.apparition, o.tf
				));

				if idx + 1 >= objs.len() {
					break;
				}
			}

			q.pop();
			q.push(';');

			stmt.execute_batch(&q).unwrap();
		}

		stmt.commit().expect("Error in commit");
	}
}
