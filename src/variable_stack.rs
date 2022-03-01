
use crate::tokenizer::Token;

use crate::variables::Variable;

#[derive(Debug)]
pub struct VariableStack {
	is_valid: bool,
	stack: Vec< Variable >, 
}

impl VariableStack {
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

	pub fn push( &mut self, variable: Variable ) {
		self.stack.push( variable );
	}

	pub fn top( &self ) -> Option< &Variable > {
		self.stack.last()
	}
	pub fn pop( &mut self ) -> Option< Variable > {
		self.stack.pop()
	}

	pub fn pop_as_f32( &mut self ) -> f32 {
		match self.stack.pop() {
			Some( Variable::I32( i ) ) => i as f32,
			Some( Variable::F32( f ) ) => f,
			_ => {
				self.is_valid = false;
				0.0
			}// panic!( "Stack top not representable as f32" ),
		}
	}

	pub fn pop_as_i32( &mut self ) -> i32 {
		match self.stack.pop() {
			Some( Variable::I32( i ) ) => i,
			Some( Variable::F32( f ) ) => f as i32,				// :TODO: decide if this is a good idea
			_ => {
				self.is_valid = false;
				0
			} //panic!( "Stack top not representable as i32" ),
		}
	}

	pub fn pop_as_string( &mut self ) -> String {
		match self.stack.pop() {
			Some( Variable::I32( i ) ) => format!("{}", i ),
			Some( Variable::F32( f ) ) => format!("{}", f ),
			Some( Variable::String( s ) ) => s,
			_ => {
				self.is_valid = false;
				String::default()
			}// panic!( "Stack top not representable as string" ),
		}
	}
}

