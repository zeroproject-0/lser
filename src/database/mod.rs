use self::models::{doc_word::DbDocWordToSave, ToQuery};

pub mod models;
pub mod sqlite;

pub trait DataBase {
	fn save<T: ToQuery>(&self, obj: T) -> usize;
	fn save_all<T: ToQuery>(&self, objs: &[T], table: String, extend_query: String);
	fn save_all_doc_word(&self, objs: &Vec<DbDocWordToSave>);
}
