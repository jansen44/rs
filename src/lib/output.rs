use crate::terminfo::TermDimensions;
use std::{fmt, fmt::Debug};

pub struct Entry {
	pub content: String,
	pub length: usize,
}

impl Debug for Entry {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.content, self.length)			
    }
}

impl Entry {
	pub fn new(content: String) -> Self {
		Entry {
			length: content.len(),
			content: content,
		}
	}
}

#[derive(Default)]
pub struct Output {
	pub entries: Vec<Entry>,
	pub longest_length: usize,
	pub columns: usize,
	pub length_sum: usize,
	pub term_dimensions: TermDimensions
}

impl Debug for Output {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
			f, 
			"entries: {:?}\n\
			columns: {}\n\
			longest_length: {}\n\
			length_sum: {}\n\
			term_dimensions: {:?}", 
			self.entries,
			self.columns,
			self.longest_length,
			self.length_sum,
			self.term_dimensions
		)
    }
}

impl Output {
	pub fn new(term_dimensions: TermDimensions) -> Self {
		Output { 
			term_dimensions,
			..Default::default()
		}
	}

	pub fn add(&mut self, entry: Entry) {
		self.length_sum += entry.length;
		if self.longest_length < entry.length {
			self.longest_length = entry.length;
		}
		self.columns = self.term_dimensions.cols / self.longest_length;
		self.entries.push(entry);
	}
}
