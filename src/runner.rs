
use crate::converter::Converter;
use crate::tokenizer::Token;

pub struct Runner {
	tokens: Vec< Token >,
}

impl Runner {
	pub fn new( buffer: &str ) -> Self {
		let mut converter = Converter::new( buffer );
		let tokens = converter.to_postfix( );

		Self {
			tokens,
		}
	}

/* for reference only
	OperandI32( i32 ),
	OperandF32( f32 ),
	Operator( Operator ),
	BraceLeft,
	BraceRight,
	Whitespace,
	EOF,
	ERROR( &'static str ),
*/
	pub fn run( &self ) -> f32 {
		let result = 0.0;
		let mut stack: Vec<Token> = Vec::new();
		for token in &self.tokens {
			match token {
				Token::OperandI32( _ ) => {
					stack.push( token.clone() );
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
								( Token::OperandI32( a ), Token::OperandI32( b ) ) => {
									let r = a + b;
									stack.push( Token::OperandI32( r ) );
								},
								( Token::OperandF32( a ), Token::OperandI32( b ) ) => {
									let r = a + *b as f32;
									stack.push( Token::OperandF32( r ) );
								},
								( Token::OperandI32( a ), Token::OperandF32( b ) ) => {
									let r = *a as f32 + b;
									stack.push( Token::OperandF32( r ) );
								},
								( Token::OperandF32( a ), Token::OperandF32( b ) ) => {
									let r = a + b;
									stack.push( Token::OperandF32( r ) );
								},
								_ => todo!("Operand combination {:?} {:?} for ADD", operand_a, operand_b ),
							}
						},
						"*" => {
							let operand_b = stack.pop().unwrap();
							let operand_a = stack.pop().unwrap();
							match ( &operand_a, &operand_b ) {
								( Token::OperandI32( a ), Token::OperandI32( b ) ) => {
									let r = a * b;
									stack.push( Token::OperandI32( r ) );
								},
								_ => todo!("Operand combination {:?} {:?} for ADD", operand_a, operand_b ),
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
		if stack.len() == 1 {
			let r = stack.pop().unwrap();
			match r {
				Token::OperandI32( i ) => {
					return i as f32;
				},
				Token::OperandF32( f ) => {
					return f;
				},
				_ => return 0.0,
			}
		}
		result
	}
}

