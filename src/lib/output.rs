use crate::terminfo::TermDimensions;
use std::{fmt, fmt::Debug};

#[derive(Clone,Eq,Ord,PartialEq,PartialOrd)]
pub struct Entry {
	pub content: String,
	pub is_dir: bool,
	pub length: usize,
}

impl Debug for Entry {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.content, self.length)
    }
}

impl fmt::Display for Entry {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl Entry {
	pub fn new(content: String, is_dir: bool) -> Self {
		Entry {
			length: content.len(),
			content,
			is_dir
		}
	}
}

#[derive(Default)]
pub struct Output {
	pub entries: Vec<Entry>,
	pub dir_entries: Vec<Entry>,
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
		if entry.is_dir {
			self.dir_entries.push(entry);
		} else {
			self.entries.push(entry);
		}
	}

	pub fn print(&mut self, dir_first: bool) {		
		let output_entries = Self::get_output_entries(
			&mut self.dir_entries, 
			&mut self.entries, 
			dir_first
		);

		for entry in output_entries {
			print!("{} ", entry);
		}

		println!();
	}

	fn get_output_entries(
		dir_entries: &mut Vec<Entry>, 
		entries: &mut Vec<Entry>,
		dir_first: bool
	) -> Vec<Entry> {
		if dir_first {
			dir_entries.sort();	entries.sort();
			dir_entries.append(entries);
			return dir_entries.clone();
		}
		dir_entries.append(entries);
		dir_entries.sort();	dir_entries.clone()
	}
}
