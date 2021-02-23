
use crate::scanner::Scanner;
use crate::operator::{Operator,OPERATORS};

#[derive(Debug,PartialEq,Eq)]
pub enum Token {
	OperandI32( i32 ),
	Operator( Operator ),
	Whitespace,
	EOF,
	ERROR,
}

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

	fn next_whitespace( &mut self ) -> bool {
		let mut had_whitespace = false;
		while Tokenizer::is_whitespace( self.scanner.peek() ) {
			had_whitespace = true;
			self.scanner.pop();
		}
		had_whitespace
	}

	pub fn next( &mut self ) -> Token {
		if self.empty() {
			Token::EOF
		} else if self.next_whitespace() {
			Token::Whitespace
		} else if let Some( o ) = self.next_operator() {
			Token::Operator( o )
		} else if let Some( i ) = self.next_i32() {
			Token::OperandI32( i )
		} else {
			Token::ERROR
		}
	}

	pub fn scanner( &self ) -> &'a Scanner {
		&self.scanner
	}
}
