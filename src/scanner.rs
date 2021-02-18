

pub struct Scanner<'a> {
	buffer: &'a str,
	pos: usize,		// :TODO: probably should use a char iterator to support utf-8
}

impl<'a> Scanner<'a> {
	pub fn new( buffer: &'a str ) -> Self {
		Self {
			buffer: buffer,
			pos: 0,
		}
	}

	pub fn empty( &self ) -> bool {
		self.pos >= self.buffer.len() 
	}

	pub fn cursor( & self ) -> usize {
		self.pos
	}

	pub fn peek( &self ) -> &str {
		if !self.empty() {
			let s = self.pos;
			let e = self.pos + 1;
			&self.buffer[ s..e ]
		} else {
			""
		}
	}

	pub fn pop( &mut self ) {
		if self.pos < self.buffer.len() {
			self.pos += 1;
		}
	}
}

