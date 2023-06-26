use super::ToQuery;

pub struct DbWord {
	pub id: usize,
	pub word: String,
	pub apparitions: usize,
}

impl ToQuery for DbWord {
	fn to_query(&self) -> String {
		String::from(format!(
			"INSERT INTO T_WORD (word, apparitions) VALUES (\"{}\", {}) ON CONFLICT (word) DO UPDATE SET apparitions = + 1",
			self.word, self.apparitions
		))
	}

	fn to_mult_query(&self) -> String {
		String::from(format!("(\"{}\", {})", self.word, self.apparitions))
	}

	fn fields(&self) -> String {
		String::from(format!("(word, apparitions)"))
	}
}
