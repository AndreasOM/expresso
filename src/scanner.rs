

use std::str::CharIndices;

#[derive(Debug)]
pub struct Scanner<'a> {
	buffer: &'a str,
	char_indices: CharIndices<'a>,
	current_char: Option<(usize, char)>,
}

impl<'a> Scanner<'a> {
	pub fn new( buffer: &'a str ) -> Self {
		let mut char_indices = buffer.char_indices();
		let current_char = char_indices.next();
		Self {
			buffer,
			char_indices,
			current_char,
		}
	}

	pub fn empty( &self ) -> bool {
		self.current_char == None
	}

	pub fn cursor( & self ) -> usize {
		if let Some( (pos,_) ) = self.current_char {
			pos
		} else {
			self.buffer.len()
		}
	}

	pub fn peek( &self ) -> &str {
		if let Some( (pos,c) ) = self.current_char {
			let e = pos + c.len_utf8();
			&self.buffer[ pos..e ]
		} else {
			""
		}
	}

	pub fn pop( &mut self ) {
		self.current_char = self.char_indices.next();
	}
}

