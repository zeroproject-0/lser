pub mod doc_word;
pub mod document;
pub mod word;

pub trait ToQuery {
	fn to_query(&self) -> String;
	fn to_mult_query(&self) -> String;
	fn fields(&self) -> String;
}
