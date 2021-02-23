
use std::env;

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
					panic!("");
				},
				_ => {},

			}
		}
	}
}
