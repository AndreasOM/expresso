
use std::env;

use expresso::expression::Expression;
use expresso::machine::Machine;
use expresso::scanner::Scanner;
use expresso::tokenizer::{ Token, Tokenizer };
use expresso::variable_storage::VariableStorage;
use expresso::variables::Variable;

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

		let function_table = machine.get_mut_function_table();

		function_table.register(
			"sin",
			|argc, variable_stack, _variable_storage| {
				// :TODO: handle wrong argc

				let fv = variable_stack.pop_as_f32();

				let r = fv.sin();
				println!( "sin called with {} arguments: {} -> {}", argc, fv, r );

				variable_stack.push( Variable::F32( r ) );
				true
			}
		);

		function_table.register(
			"setVar",
			|argc, variable_stack, variable_storage| {
				// :TODO: handle wrong argc

				let v = variable_stack.pop();
				let n = variable_stack.pop();
				
				match (&n, &v) {
					( Some( Variable::String( n ) ), Some( v ) ) => {
						println!( "setVar called with {} arguments: {} -> {:?}", argc, n, v );
						variable_storage.set( &n, v.clone() );
						true

					},
					_ => {
						todo!("setVar( {:?}, {:?} )", n, v );
						false
					},
				}
			}
		);

		let mut expression = Expression::new();
		expression.from_str( &argument );
		println!("{}", expression);
		let mut r = expression.run( &mut machine );
		match r.pop() {
			Some( Variable::I32( i ) ) => {
				println!("{}", i );
			},
			Some( Variable::F32( f ) ) => {
				println!("{}", f );
			},
			None => println!("No result" ),
			r => todo!("Result is not printable {:?}", r ),
		}

		machine.save_variable_storage( variable_filename ).unwrap();
	}
}
