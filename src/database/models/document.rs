use super::ToQuery;

pub struct DbDocument {
	pub id: usize,
	pub path: String,
	pub total_terms: usize,
}

impl ToQuery for DbDocument {
	fn to_query(&self) -> String {
		String::from(format!(
			"INSERT INTO T_DOCUMENT (path, total_terms) VALUES (\"{}\", {})",
			self.path, self.total_terms
		))
	}

	fn to_mult_query(&self) -> String {
		String::from(format!("({}, {})", self.path, self.total_terms))
	}

	fn fields(&self) -> String {
		String::from(format!("(path, total_terms)"))
	}
}
