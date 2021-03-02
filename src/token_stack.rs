
use crate::tokenizer::Token;

pub struct TokenStack {
	stack: Vec< Token >, 
}

impl TokenStack {
	pub fn new() -> Self {
		Self {
			stack: Vec::new(),
		}
	}

	pub fn empty( &self ) -> bool {
		self.stack.len() == 0
	}

	pub fn len( &self ) -> usize {
		self.stack.len()
	}

	pub fn push( &mut self, token: Token ) {
		self.stack.push( token );
	}

	pub fn pop( &mut self ) -> Option< Token > {
		self.stack.pop()
	}
}

