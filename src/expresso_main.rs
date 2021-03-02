
use std::env;

use expresso::converter::Converter;
use expresso::expression::Expression;
use expresso::scanner::Scanner;
use expresso::tokenizer::{ Token, Tokenizer };

fn main() {

	for argument in env::args().skip(1) {
		println!("{}", argument);
		let scanner = Scanner::new( &argument );
		let mut tokenizer = Tokenizer::new( scanner );
		while !tokenizer.empty() {
//			dbg!(tokenizer.scanner());
			let t = tokenizer.next();
			println!("{:?}", t);

			match t {
				Token::ERROR( s ) => {
					print!("Error tokenizing {}", &s );
//					panic!("");
					break;
				},
				_ => {},

			}
		}

		println!("\n----\n");
		let mut converter = Converter::new( &argument );
		let postfix = converter.to_postfix( );

		dbg!( &postfix );
		let mut expression = Expression::new();
		expression.from_str( &argument );
		println!("{}", expression.result_as_i32_or( 0 ) );
	}
}
