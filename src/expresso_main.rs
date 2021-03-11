
use std::env;

use expresso::expression::Expression;
use expresso::machine::Machine;
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
		let mut machine = Machine::new();
		let variable_filename = "variables.yaml";	// :TODO: allow command line override

		machine.load_variable_storage( variable_filename );

		let mut expression = Expression::new();
		expression.from_str( &argument );
		println!("{}", expression);
		match expression.result_as_i32( &mut machine ) {
			Some( r ) => println!("{}", r ),
			None => println!("Result is not representable as I32" ),
		}

		machine.save_variable_storage( variable_filename );
	}
}
