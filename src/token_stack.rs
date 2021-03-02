
use crate::tokenizer::Token;

pub struct TokenStack {
	is_valid: bool,
	stack: Vec< Token >, 
}

impl TokenStack {
	pub fn new() -> Self {
		Self {
			is_valid: true,
			stack: Vec::new(),
		}
	}

	pub fn is_valid(&self) -> bool {
		self.is_valid
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

	pub fn pop_as_f32( &mut self ) -> f32 {
		match self.stack.pop() {
			Some( Token::OperandI32( i ) ) => i as f32,
			Some( Token::OperandF32( f ) ) => f,
			_ => {
				self.is_valid = false;
				0.0
			}// panic!( "Stack top not representable as f32" ),
		}
	}

	pub fn pop_as_i32( &mut self ) -> i32 {
		match self.stack.pop() {
			Some( Token::OperandI32( i ) ) => i,
			Some( Token::OperandF32( f ) ) => f as i32,				// :TODO: decide if this is a good idea
			_ => {
				self.is_valid = false;
				0
			} //panic!( "Stack top not representable as i32" ),
		}
	}
}

