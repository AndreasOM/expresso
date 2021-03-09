
use crate::scanner::Scanner;
use crate::operator::{Operator,OPERATORS};

#[derive(Debug,PartialEq,Clone)]
pub enum Token {
	Literal( String ),
	OperandI32( i32 ),
	OperandF32( f32 ),
	StringLiteral( String ),
	Variable( String ),
	Operator( Operator ),
	BraceLeft,
	BraceRight,
	Whitespace,
	EOF,
	ERROR( &'static str ),

	// :HACK: clean me up
	FunctionCall,
}

#[derive(Debug)]
pub struct Tokenizer<'a> {
	scanner: Scanner<'a>,
}

impl<'a> Tokenizer<'a> {
	pub fn new( scanner: Scanner<'a> ) -> Self {
		Self {
			scanner,
		}
	}

	pub fn empty( &self ) -> bool {
		self.scanner.empty()
	}

	fn as_digit( s: &str ) -> Option< i32 > {
		match s {
			"0" => Some( 0 ),
			"1" => Some( 1 ),
			"2" => Some( 2 ),
			"3" => Some( 3 ),
			"4" => Some( 4 ),
			"5" => Some( 5 ),
			"6" => Some( 6 ),
			"7" => Some( 7 ),
			"8" => Some( 8 ),
			"9" => Some( 9 ),
			_ => None,
		}
	}

	fn is_allowed_in_literal( s: &str ) -> bool {
		if Tokenizer::is_alphanumeric( s ) {
			true
		} else {
			["_"].contains( &s )
		}
	}
	fn is_alphabetic( s: &str ) -> bool {
		let mut chars = s.chars();
		if let Some( c ) = chars.next() {
			if c.is_ascii_alphabetic() {
				true
			} else {
				false
			}
		} else {
			false
		}
	}

	fn is_alphanumeric( s: &str ) -> bool {
		let mut chars = s.chars();
		if let Some( c ) = chars.next() {
			if c.is_ascii_alphanumeric() {
				true
			} else {
				false
			}
		} else {
			false
		}
	}

	fn is_whitespace( s: &str ) -> bool {
		match s {
			" " => true,
			_ => false,
		}
	}

	// :HACK: !!!!!!!!!!!!!!!!!
	fn next_operator( &mut self ) -> Option< Operator > { // :HACK: resolve operator handling next time
		let s = self.scanner.peek();	// :HACK: and more hacking, we need multi character operators
		for o in OPERATORS.iter() {
			if o.literal == s {
				self.scanner.pop();
				return Some( o.clone() )
			}
		}
		None
	}

	fn next_i32( &mut self ) -> Option< i32 > {
		let mut c = self.scanner.peek();
		let mut v = 0;
		let mut is_valid = false;

		while let Some( d ) = Tokenizer::as_digit( c ) {
			v = v * 10 + d;
			is_valid = true;
			self.scanner.pop();
			c = self.scanner.peek();
		};

		if is_valid {
			Some( v )
		} else {
			None
		}
	}

	fn get_number_of_digits( n: i32 ) -> i32 {
		if n > 999_999_999 {
			10
		} else if n > 99_999_999 {
			9
		} else if n > 9_999_999 {
			8
		} else if n > 999_999 {
			7
		} else if n > 99_999 {
			6
		} else if n > 9_999 {
			5
		} else if n > 999 {
			4
		} else if n > 99 {
			3
		} else if n > 9 {
			2
		} else {
			1
		}
	}

	fn next_number( &mut self ) -> Option< Token > {
		if let Some( i ) = self.next_i32() {
			if "." == self.scanner.peek() {
				self.scanner.pop();
				if let Some( j ) = self.next_i32() {
					// :HACK: but we don't want any dependencies
//					dbg!(i, j);
					let f = i as f32;
					let n = Tokenizer::get_number_of_digits( j );
					let shift = 10_f32.powf( n as f32 );
//					dbg!( &n, &shift );
					let f = f + ( j as f32 / shift );
					Some( Token::OperandF32( f ) )
				} else {
					// dot but no decimal part
					Some( Token::ERROR( "malformed float" ) )					
				}
			} else {
				Some( Token::OperandI32( i ) )
			}
		} else {
			None
		}
	}

	fn next_whitespace( &mut self ) -> bool {
		let mut had_whitespace = false;
		while Tokenizer::is_whitespace( self.scanner.peek() ) {
			had_whitespace = true;
			self.scanner.pop();
		}
		had_whitespace
	}

	fn next_literal( &mut self ) -> Option< Token > {
		let c = self.scanner.peek();
		if Tokenizer::is_alphabetic( c ) {
			let mut value = c.to_string();
			self.scanner.pop();

			let mut c = self.scanner.peek();
			while Tokenizer::is_allowed_in_literal( c ) {
				value = value + c;
				self.scanner.pop();
				c = self.scanner.peek();
			};

			Some( Token::Literal( value ) )

		} else {
			None
		}
	}

	fn next_variable( &mut self ) -> Option< Token > {
		if self.scanner.peek() == "$" {
			self.scanner.pop();

			let mut name = String::new();


			let mut c = self.scanner.peek();
			while Tokenizer::is_alphanumeric( c ) {
				name = name + c;
				self.scanner.pop();
				c = self.scanner.peek();
			};

			if name.len() > 0 {
				Some( Token::Variable( name ) )
			} else {
				Some( Token::ERROR("Missing variable name" ) )
			}
		} else {
			None
		}
	}

	fn next_brace( &mut self ) -> Option< Token > {
		let c = self.scanner.peek();
		match c {
			"(" => {
				self.scanner.pop();
				Some( Token::BraceLeft )
			},
			")" => {
				self.scanner.pop();
				Some( Token::BraceRight )
			},
			_ => None,
		}
	}

	fn next_string_literal( &mut self ) -> Option< Token > {
		if self.scanner.peek() == "\"" {
			self.scanner.pop();

			let mut value = String::new();

			let mut c = self.scanner.peek();
			while c != "" {
//				dbg!(&self.scanner);
				if c != "\"" {
					value = value + c;
					self.scanner.pop();
				} else {			// closing quote found		
					self.scanner.pop();
					return Some( Token::StringLiteral( value ) )
				}
				c = self.scanner.peek();
			};
			Some( Token::ERROR( "Unterminated string found" ) )
		} else {
			None
		}
	}

	pub fn next( &mut self ) -> Token {
		if self.empty() {
			Token::EOF
		} else if self.next_whitespace() {
			Token::Whitespace
		} else if let Some( l ) = self.next_literal() {
			l
		} else if let Some( v ) = self.next_variable() {
			v
		} else if let Some( s ) = self.next_string_literal() {
			s
		} else if let Some( o ) = self.next_brace() {
			o
		} else if let Some( o ) = self.next_operator() {
			Token::Operator( o )
		} else if let Some( n ) = self.next_number() {
			n
		} else {
			Token::ERROR( "unhandled token" )
		}
	}

	pub fn scanner( &self ) -> &'a Scanner {
		&self.scanner
	}
}
