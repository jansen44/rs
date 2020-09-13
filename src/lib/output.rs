use crate::terminfo::TermDimensions;
use crate::color::{Colors};
use std::fmt;

#[derive(Clone,Eq,Ord,PartialEq,PartialOrd)]
pub struct Entry {
	pub content: String,
	pub is_dir: bool,
	pub length: usize,
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
		if self.length_sum > self.term_dimensions.cols {
			Self::print_grid(
				&output_entries, 
				self.columns, 
				self.longest_length
			);
		} else {
			Self::print_line(&output_entries);
		}
	}

	fn print_grid(
		entries: &Vec<Entry>, 
		columns: usize, 
		longest_length: usize
	) {
		let mut entries_iter = entries.iter().peekable();
		while entries_iter.peek() != None {
			let mut i = 0;
			while i < columns && entries_iter.peek() != None {
				Output::colorful_output(
					entries_iter.next().unwrap(), 
					Some(longest_length)
				);
				i += 1;
			}
			println!();
		}
	}

	fn print_line(entries: &Vec<Entry>) {
		for entry in entries {
			Output::colorful_output(entry, None);
		}
		println!();
	}

	fn colorful_output(entry: &Entry, width: Option<usize>) {
		if let Some(width) = width {
			print!(
				"{}{content:<width$} ",
				Output::get_fg_color(entry),
				content=entry.content,
				width=width
			);
		} else {
			print!("{}{} ", Output::get_fg_color(entry), entry);
		}
	}

	fn get_fg_color(entry: &Entry) -> String {
		if entry.is_dir { 
			return Colors::LightCyan.fg() 
		}
		Colors::White.fg()
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
