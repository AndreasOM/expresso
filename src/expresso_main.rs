
use std::env;

use expresso::expression::Expression;
use expresso::scanner::Scanner;
use expresso::tokenizer::{ Token, Tokenizer };
use expresso::variable_storage::VariableStorage;

fn main() {

	for argument in env::args().skip(1) {
		println!("Argument: >>{}<<", argument);
		let scanner = Scanner::new( &argument );
		let mut tokenizer = Tokenizer::new( scanner );
		while !tokenizer.empty() {
//			dbg!(tokenizer.scanner());
			let t = tokenizer.next();
			println!("!! {:?}", t);

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
		let variable_filename = "variables.yaml";	// :TODO: allow command line override

		let mut variable_storage = if let Ok( vs ) = VariableStorage::load( variable_filename ) {
			vs
		} else {
			VariableStorage::new()
		};

//		variable_storage.set_i32( "var1", 44 );
//		dbg!(&variable_storage);
		let mut expression = Expression::new();
		expression.from_str( &argument );
		println!("{}", expression);
		match expression.result_as_i32( &mut variable_storage ) {
			Some( r ) => println!("{}", r ),
			None => println!("Result is not representable as I32" ),
		}

		variable_storage.save( variable_filename );
	}
}
