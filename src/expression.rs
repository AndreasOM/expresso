
use core::fmt::Formatter;

use crate::converter::Converter;
use crate::instructions::Instruction;
use crate::machine::Machine;
use crate::variable_stack::VariableStack;
use crate::tokenizer::Token;
use crate::variable_storage::VariableStorage;
use crate::variables::Variable;

#[derive(Debug)]
pub struct Expression {
	is_valid: bool,
	instructions: Vec<Instruction>,
	upgrade_literals_to_strings: bool,
}

impl Expression {
	pub fn new() -> Self {
		Self {
			is_valid: true,
			instructions: Vec::new(),
			upgrade_literals_to_strings: false,
		}
	}

	pub fn enable_upgrade_of_literals_to_strings( &mut self ) {
		self.upgrade_literals_to_strings = true;
	}

	pub fn from_str( &mut self, buffer: &str ) -> anyhow::Result<()> {
		self.is_valid = false;	// asume invalid until proven otherwise
		let mut converter = Converter::new( buffer );
		if self.upgrade_literals_to_strings {
			converter.enable_upgrade_of_literals_to_strings();
		}
		self.instructions = converter.to_postfix( )?;
		self.validate();
		Ok(())
	}

	fn validate( &mut self ) {
		// :WIP:
		let mut machine = Machine::new();
		let result = self.run( &mut machine );
		if result.len() != 1 {
			println!( "Expression doesn't have ONE result" );
			self.is_valid = false;
		} else if !result.is_valid() {
			println!( "Expression mangels token stack" ); // :TODO: better error reporting
			dbg!(&result);
			self.is_valid = false;
		} else {
			self.is_valid = true;
		}
	}

	pub fn is_valid( &self ) -> bool {
		self.is_valid
	}

	pub fn result_as_i32( &self, machine: &mut Machine ) -> Option<i32> {
		let mut result = self.run( machine );

		match result.pop() {
			Some( Variable::I32( i ) ) => Some( i ),
			Some( Variable::F32( f ) ) => Some( f as i32 ),
			_ => None,
		}		
	}

	pub fn result_as_i32_or( &self, machine: &mut Machine, default: i32 ) -> i32 {
		if self.is_valid {
			self.result_as_i32( machine ).unwrap_or( default )
		} else {
			default
		}
	}

	// Note: This assumes a valid expression
	pub fn run( &self, machine: &mut Machine ) -> VariableStack {
		let mut stack = VariableStack::new();
		for instruction in &self.instructions {
			match instruction {
				Instruction::PushI32( i ) => {
					stack.push( Variable::F32( *i as f32 ) ); // cheat, and do all calculations based on f32
				},
				Instruction::PushF32( f ) => {
					stack.push( Variable::F32( *f ) );
				},
				Instruction::PushVariable( name ) => {
					//println!("Expanding variable {}", name );
					//stack.push( token.clone() );
					let variable_storage = machine.get_mut_variable_storage();
					match variable_storage.get( name ) {
						Some( v ) => stack.push( v.clone() ),
						_ => stack.push( Variable::ERROR( format!("Variable `{}` not found", &name).to_string() ) ),
					}
				},
				Instruction::PushString( s ) => {
					stack.push( Variable::String( s.clone() ) );
				},
				Instruction::StartList => {
					stack.push( Variable::List( 0 ) );
				},
				Instruction::EndList => {
					let t = stack.top();

					match t {
						Some( Variable::List( _ ) ) => {
							// top of stack is a list, nothing to do here
						},
						_ => {
							let b = stack.pop();
							let a = stack.pop();
							match ( &a, &b ) {
								( Some( Variable::List( n ) ), Some( b ) ) => {
									stack.push( b.clone() );
									stack.push( Variable::List( n+1 ) );
								},
								_ => todo!( "EndList with {:?} {:?}", a, b ),
							}
						}
					}
				},
				Instruction::Operator( o ) => {
					// :TODO: improved error handling -> no, since all expressions are pre validated
					match o.literal {
						"+" => {
							let b = stack.pop_as_f32( );
							let a = stack.pop_as_f32( );
							let r = a + b;
							stack.push( Variable::F32( r ) );
						},
						"-" => {
							let b = stack.pop_as_f32( );
							let a = stack.pop_as_f32( );
							let r = a - b;
							stack.push( Variable::F32( r ) );
						},
						"*" => {
							let b = stack.pop_as_f32( );
							let a = stack.pop_as_f32( );
							let r = a * b;
							stack.push( Variable::F32( r ) );
						},
						"/" => {
							let b = stack.pop_as_f32( );
							let a = stack.pop_as_f32( );
							let r = a / b;
							stack.push( Variable::F32( r ) );
						},
						"," => {
							let b = stack.pop();
							let a = stack.pop();
							let l = stack.pop();

							match ( &l, &a, &b ) {
								( Some( Variable::List( n ) ), Some( a ), Some( b ) ) => {
									stack.push( a.clone() );
									stack.push( b.clone() );
									stack.push( Variable::List( n+2 ) );
								},
								( Some( l ), Some( Variable::List( n ) ), Some( b ) ) => {
									stack.push( l.clone() );	// took this by accident just put it back
									stack.push( b.clone() );
									stack.push( Variable::List( n+1 ) );
								},
								_ => todo!(),
							}

						}
						_ => todo!("Operator {:?}", o ),
					}
				},
				Instruction::CallFunction => {
					let name = stack.pop();
					let args = stack.pop();
					match ( &name, &args ) {
						( Some( Variable::String( name ) ), Some( Variable::List( argc ) ) ) => {
							machine.call_function( name, *argc, &mut stack );
						},
						_ => panic!( "Invalid operands for CallFunction {:?}( {:?} )", name, args ),
					}

				}

				_ => {
					panic!("Error instruction {:?} should never be run", instruction );
				},
			}
		}
		stack
	}

}


impl std::fmt::Display for Expression {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		if !self.is_valid {
			f.write_fmt(format_args!("INVALID Expression!\n"))?
		};

		for t in &self.instructions {
			match t {
				Instruction::PushI32( i ) => f.write_fmt(format_args!("(I32) {}\n", *i))?,
				Instruction::PushF32( fv ) => f.write_fmt(format_args!("(F32) {}\n", *fv))?,
				Instruction::Operator( o ) => f.write_fmt(format_args!("(OPR) {}\n", o.literal))?,
				_ => f.write_fmt(format_args!("Token {:?}", t))?,
			}
			
		};
		Ok(())
	}
}

