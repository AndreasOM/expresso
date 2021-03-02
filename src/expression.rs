
use core::fmt::Formatter;

use crate::converter::Converter;
use crate::token_stack::TokenStack;
use crate::tokenizer::Token;

pub struct Expression {
	tokens: Vec<Token>,
}

impl Expression {
	pub fn new() -> Self {
		Self {
			tokens: Vec::new(),
		}
	}

	pub fn from_str( &mut self, buffer: &str ) {
		let mut converter = Converter::new( buffer );
		self.tokens = converter.to_postfix( );
	}

	pub fn result_as_i32( &self ) -> Option<i32> {
		let mut result = self.run();

		match result.pop() {
			Some( Token::OperandI32( i ) ) => Some( i ),
			Some( Token::OperandF32( f ) ) => Some( f as i32 ),
			_ => None,
		}		
	}

	pub fn result_as_i32_or( &self, default: i32 ) -> i32 {
		self.result_as_i32().unwrap_or( default )
	}

	// Note: This assumes a valid expression
	fn run( &self ) -> TokenStack {
		let mut stack = TokenStack::new();
		for token in &self.tokens {
			match token {
				Token::OperandI32( i ) => {
					stack.push( Token::OperandF32( *i as f32 ) ); // cheat, and do all calculations based on f32
				},
				Token::OperandF32( _ ) => {
					stack.push( token.clone() );
				},
				Token::Operator( o ) => {
					// :TODO: improved error handling
					match o.literal {
						"+" => {
							let b = stack.pop_as_f32( );
							let a = stack.pop_as_f32( );
							let r = a + b;
							stack.push( Token::OperandF32( r ) );
						},
						"-" => {
							let b = stack.pop_as_f32( );
							let a = stack.pop_as_f32( );
							let r = a - b;
							stack.push( Token::OperandF32( r ) );
						},
						"*" => {
							let b = stack.pop_as_f32( );
							let a = stack.pop_as_f32( );
							let r = a * b;
							stack.push( Token::OperandF32( r ) );
						},
						"/" => {
							let b = stack.pop_as_f32( );
							let a = stack.pop_as_f32( );
							let r = a / b;
							stack.push( Token::OperandF32( r ) );
						},
						_ => todo!("Operator {:?}", o ),
					}
				}
				_ => {
					panic!("Error token {:?} should never be run", token );
				},
			}
		}
		stack
	}

}


impl std::fmt::Display for Expression {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		for t in &self.tokens {
			match t {
				Token::OperandI32( i ) => f.write_fmt(format_args!("(I32) {}\n", *i))?,
				Token::OperandF32( fv ) => f.write_fmt(format_args!("(F32) {}\n", *fv))?,
				Token::Operator( o ) => f.write_fmt(format_args!("(OPR) {}\n", o.literal))?,
				_ => f.write_fmt(format_args!("Token {:?}", t))?,
			}
			
		};
		Ok(())
	}
}

