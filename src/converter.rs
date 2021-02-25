
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

	pub fn to_postfix( &mut self ) -> Vec<Token> {
		let mut result = Vec::new();
		let mut tokens: Vec<Token> = Vec::new();


		let scanner = Scanner::new( self.buffer );
		let mut tokenizer = Tokenizer::new( scanner );

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
		loop {
			let token = tokenizer.next();
			println!("{:?}", token);
//			dbg!( &token );
			match token {
				Token::OperandI32( _ ) => {
					result.push( token );
				},
				Token::OperandF32( _ ) => {
					result.push( token );
				},
				Token::Operator( ref o ) => {
					while tokens.len() > 0 {
						let top = tokens.pop().unwrap();
						match top {
							Token::Operator( ref to ) => {
								if to.precendence > o.precendence {
									tokens.push( top );
									break;
								} else {
									result.push( top );
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
				Token::Whitespace => {	// whitespace is junk, just do nothing

				},
				Token::EOF => break,
				_ => {
					todo!( "{:?}", &token )
				},
			}
		};

		while let Some( token ) = tokens.pop() {
			result.push( token );
		}

//		result.push( Token::EOF );

		result
	}

}
