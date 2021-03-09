
use core::fmt::Formatter;

use crate::converter::Converter;
use crate::instructions::Instruction;
use crate::variable_stack::VariableStack;
use crate::tokenizer::Token;
use crate::variable_storage::VariableStorage;
use crate::variables::Variable;

pub struct Expression {
	is_valid: bool,
	instructions: Vec<Instruction>,
}

impl Expression {
	pub fn new() -> Self {
		Self {
			is_valid: true,
			instructions: Vec::new(),
		}
	}

	pub fn from_str( &mut self, buffer: &str ) {
		let mut converter = Converter::new( buffer );
		self.instructions = converter.to_postfix( );
		self.validate();
	}

	fn validate( &mut self ) {
		// :WIP:
		let mut variable_storage = VariableStorage::new();
		let result = self.run( &mut variable_storage );
		if result.len() != 1 {
			println!( "Expression doesn't have ONE result" );
			self.is_valid = false;
		} else if !result.is_valid() {
			println!( "Expression mangels token stack" ); // :TODO: better error reporting
			dbg!(&result);
			self.is_valid = false;
		} else {
			self.is_valid = true;
		}
	}

	pub fn is_valid( &self ) -> bool {
		self.is_valid
	}

	pub fn result_as_i32( &self, variable_storage: &mut VariableStorage ) -> Option<i32> {
		let mut result = self.run( variable_storage );

		match result.pop() {
			Some( Variable::I32( i ) ) => Some( i ),
			Some( Variable::F32( f ) ) => Some( f as i32 ),
			_ => None,
		}		
	}

	pub fn result_as_i32_or( &self, variable_storage: &mut VariableStorage, default: i32 ) -> i32 {
		if self.is_valid {
			self.result_as_i32( variable_storage ).unwrap_or( default )
		} else {
			default
		}
	}

	// Note: This assumes a valid expression
	fn run( &self, variable_storage: &mut VariableStorage ) -> VariableStack {
		let mut stack = VariableStack::new();
		for instruction in &self.instructions {
			match instruction {
				Instruction::PushI32( i ) => {
					stack.push( Variable::F32( *i as f32 ) ); // cheat, and do all calculations based on f32
				},
				Instruction::PushF32( f ) => {
					stack.push( Variable::F32( *f ) );
				},
				Instruction::PushVariable( name ) => {
					println!("Expanding variable {}", name );
					//stack.push( token.clone() );
					match variable_storage.get( name ) {
						Some( Variable::I32( i ) ) => stack.push( Variable::I32( *i ) ),
						_ => stack.push( Variable::ERROR( "Variable not found".to_string() ) ),
					}
				},
				Instruction::Operator( o ) => {
					// :TODO: improved error handling -> no, since all expressions are pre validated
					match o.literal {
						"+" => {
							let b = stack.pop_as_f32( );
							let a = stack.pop_as_f32( );
							let r = a + b;
							stack.push( Variable::F32( r ) );
						},
						"-" => {
							let b = stack.pop_as_f32( );
							let a = stack.pop_as_f32( );
							let r = a - b;
							stack.push( Variable::F32( r ) );
						},
						"*" => {
							let b = stack.pop_as_f32( );
							let a = stack.pop_as_f32( );
							let r = a * b;
							stack.push( Variable::F32( r ) );
						},
						"/" => {
							let b = stack.pop_as_f32( );
							let a = stack.pop_as_f32( );
							let r = a / b;
							stack.push( Variable::F32( r ) );
						},
						_ => todo!("Operator {:?}", o ),
					}
				}
				_ => {
					panic!("Error instruction {:?} should never be run", instruction );
				},
			}
		}
		stack
	}

}


impl std::fmt::Display for Expression {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		if !self.is_valid {
			f.write_fmt(format_args!("INVALID Expression!\n"))?
		};

		for t in &self.instructions {
			match t {
				Instruction::PushI32( i ) => f.write_fmt(format_args!("(I32) {}\n", *i))?,
				Instruction::PushF32( fv ) => f.write_fmt(format_args!("(F32) {}\n", *fv))?,
				Instruction::Operator( o ) => f.write_fmt(format_args!("(OPR) {}\n", o.literal))?,
				_ => f.write_fmt(format_args!("Token {:?}", t))?,
			}
			
		};
		Ok(())
	}
}

