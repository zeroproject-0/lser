use super::ToQuery;

#[derive(Clone)]
pub struct DbDocWordToSave {
	pub word: String,
	pub id_doc: usize,
	pub apparition: usize,
	pub tf: f32,
}
// pub type DbDocWordToSave = (String, usize, usize, f32);

pub struct DbDocWord {
	pub id: usize,
	pub id_doc: usize,
	pub id_word: usize,
	pub apparition: usize,
	pub tf: f32,
}

impl ToQuery for DbDocWord {
	fn to_query(&self) -> String {
		String::from(format!(
			"INSERT INTO T_DOC_WORD (id_doc, id_word, apparition, tf) VALUES ({}, {}, {}, {})",
			self.id_doc, self.id_word, self.apparition, self.tf
		))
	}

	fn to_mult_query(&self) -> String {
		String::from(format!(
			"({}, {}, {}, {})",
			self.id_doc, self.id_word, self.apparition, self.tf
		))
	}

	fn fields(&self) -> String {
		String::from(format!("(id_doc, id_word, apparition, tf)"))
	}
}
