
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

	pub fn result_as_i32_or( &self, default: i32 ) -> i32 {
		let mut result = self.run();

		match result.pop() {
			Some( Token::OperandI32( i ) ) => i,
			Some( Token::OperandF32( f ) ) => f as i32,
			_ => default,
		}
	}

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
							let operand_b = stack.pop().unwrap();
							let operand_a = stack.pop().unwrap();
							match ( &operand_a, &operand_b ) {
								( Token::OperandF32( a ), Token::OperandF32( b ) ) => {
									let r = a + b;
									stack.push( Token::OperandF32( r ) );
								},
								_ => todo!("Operand combination {:?} {:?} for ADD", operand_a, operand_b ),
							}
						},
						"-" => {
							let operand_b = stack.pop().unwrap();
							let operand_a = stack.pop().unwrap();
							match ( &operand_a, &operand_b ) {
								( Token::OperandF32( a ), Token::OperandF32( b ) ) => {
									let r = a - b;
									stack.push( Token::OperandF32( r ) );
								},
								_ => todo!("Operand combination {:?} {:?} for SUBTRACT", operand_a, operand_b ),
							}
						},
						"*" => {
							let operand_b = stack.pop().unwrap();
							let operand_a = stack.pop().unwrap();
							match ( &operand_a, &operand_b ) {
								( Token::OperandF32( a ), Token::OperandF32( b ) ) => {
									let r = a * b;
									stack.push( Token::OperandF32( r ) );
								},
								_ => todo!("Operand combination {:?} {:?} for MULTIPLY", operand_a, operand_b ),
							}
						},
						"/" => {
							let operand_b = stack.pop().unwrap();
							let operand_a = stack.pop().unwrap();
							match ( &operand_a, &operand_b ) {
								( Token::OperandF32( a ), Token::OperandF32( b ) ) => {
									let r = a / b;
									stack.push( Token::OperandF32( r ) );
								},
								_ => todo!("Operand combination {:?} {:?} for DIVIDE", operand_a, operand_b ),
							}
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
