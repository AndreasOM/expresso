
use crate::instructions::Instruction;
use crate::scanner::Scanner;
use crate::tokenizer::{Token, Tokenizer};

pub struct Converter<'a> {
	buffer: &'a str,
}

impl <'a>Converter<'a> {

	pub fn new( buffer: &'a str ) -> Self {
		Self {
			buffer,
		}
	}

	fn token_to_instruction( token: &Token ) -> Option< Instruction > {
		match token {
			Token::OperandI32( i ) => {
				Some( Instruction::PushI32( *i ) )
			},
			Token::OperandF32( f ) => {
				Some( Instruction::PushF32( *f ) )
			},
			Token::Variable( v ) => {
				Some( Instruction::PushVariable( v.clone() ) )
			},
			Token::Operator( ref o ) => {
				Some( Instruction::Operator( o.clone() ) )
			},

			_ => None, //panic!( "Cannot convert token {:?} to instruction", token );						
		}
	}

	pub fn to_postfix( &mut self ) -> Vec<Instruction> {
		let mut result: Vec< Instruction > = Vec::new();
		let mut tokens: Vec< Token > = Vec::new();


		let scanner = Scanner::new( self.buffer );
		let mut tokenizer = Tokenizer::new( scanner );

		loop {
			let token = tokenizer.next();
//			println!("{:?}", token);
			match token {
				Token::Literal( _ ) => {
					dbg!(&result, &tokens);
					todo!("literal");
				}
				Token::Operator( ref o ) => {
					while tokens.len() > 0 {
						let top = tokens.pop().unwrap();
						match top {
							Token::Operator( ref to ) => {
								if to.precendence > o.precendence {
									tokens.push( top );
									break;
								} else {
									result.push( Instruction::Operator( to.clone() ) );
								}
							},
							_ => {
								tokens.push( top );
								break;
							}
						}
					}
					if o.is_right_associative {
						todo!("Right associative operators");
					};

					tokens.push( token );
				},
				Token::BraceLeft => {
					tokens.push( token );
				},
				Token::BraceRight => {
					let mut left_brace_found = false;
					while tokens.len() > 0 {
						let to = tokens.pop().unwrap();
						match to {
							Token::BraceLeft => {
								left_brace_found = true;
								break;
							},
							_ => {
								if let Some( i ) = Converter::token_to_instruction( &to ) {
									result.push( i );
								} else {
									todo!("convert token {:?} to instruction", &to );
								}
							},
						}
					};

					if !left_brace_found {
						todo!("Handle missing left brace");
					}
				}
				Token::Whitespace => {	// whitespace is junk, just do nothing

				},
				Token::EOF => break,
				Token::ERROR( e ) => {
					todo!( "{:?}", e );
				},
				_ => {
					if let Some( i ) = Converter::token_to_instruction( &token ) {
						result.push( i );
					} else {
						todo!("convert token {:?} to instruction", &token );
					}					
				},
			}
		};

		while let Some( token ) = tokens.pop() {
			if let Some( i ) = Converter::token_to_instruction( &token ) {
				result.push( i );
			} else {
				todo!("convert token {:?} to instruction", &token );
			}
		}

//		result.push( Token::EOF );

		result
	}

}
