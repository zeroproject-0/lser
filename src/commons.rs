use std::process::exit;

pub struct Lexer<'a> {
	pub content: &'a [char],
}

impl<'a> Lexer<'a> {
	pub fn new(content: &'a [char]) -> Self {
		Self { content }
	}

	fn trim_left(&mut self) {
		while !self.content.is_empty() && !self.content[0].is_alphanumeric() {
			self.content = &self.content[1..];
		}
	}

	pub fn next_token(&mut self) -> Option<String> {
		self.trim_left();
		if self.content.is_empty() {
			return None;
		}

		let mut n = 0;

		while n < self.content.len() && self.content[n].is_alphanumeric() {
			n += 1;
		}

		let result = &self.content[0..n];
		self.content = &self.content[n..];

		Some(
			result
				.iter()
				.map(|c| c.to_ascii_uppercase())
				.collect::<String>(),
		)
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		self.next_token()
	}
}

fn help() {
	println!("lser:");
}

pub fn exit_error(message: &str) {
	eprintln!("ERROR: {}", message);
	help();
	exit(1);
}
