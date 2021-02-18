

use super::scanner::Scanner;
use super::tokenizer::{Token, Tokenizer};

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn scanner_checks_empty() {
	let scanner = Scanner::new( "" );

	assert_eq!( scanner.empty(), true );
}

#[test]
fn scanner_checks_not_empty() {
	let scanner = Scanner::new( "a" );

	assert_eq!( scanner.empty(), false );
}

#[test]
fn scanner_peeks_and_pops() {
	let mut scanner = Scanner::new( "abcdef" );

	assert_eq!( scanner.empty(), false );
	assert_eq!( scanner.peek(), "a" );
	scanner.pop();
	assert_eq!( scanner.peek(), "b" );
	scanner.pop();
	assert_eq!( scanner.peek(), "c" );
	scanner.pop();
	assert_eq!( scanner.peek(), "d" );
	scanner.pop();
	assert_eq!( scanner.peek(), "e" );
	scanner.pop();
	assert_eq!( scanner.peek(), "f" );
	scanner.pop();

	assert_eq!( scanner.empty(), true );
}

#[test]
fn scanner_peeks_on_empty() {
	let scanner = Scanner::new( "" );

	assert_eq!( scanner.peek(), "" );
}

#[test]
fn scanner_returns_correct_position() {
	let mut scanner = Scanner::new( "ab" );

	assert_eq!( scanner.cursor(), 0 );
	scanner.pop();
	assert_eq!( scanner.cursor(), 1 );
	scanner.pop();
	assert_eq!( scanner.cursor(), 2 );
	scanner.pop();
	assert_eq!( scanner.cursor(), 2 );
	scanner.pop();
	assert_eq!( scanner.cursor(), 2 );

	let mut scanner = Scanner::new( "" );
	assert_eq!( scanner.cursor(), 0 );
	scanner.pop();
	assert_eq!( scanner.cursor(), 0 );

}

#[test]
fn scanner_works_with_braille() {
	let mut scanner = Scanner::new( "⡌⠁⠧⠑ ⠼⠁⠒  ⡍⠜⠇⠑⠹⠰⠎ ⡣⠕⠌" );

	assert_eq!( scanner.peek(), "⡌" );
	scanner.pop();
	assert_eq!( scanner.cursor(), 3 );
	assert_eq!( scanner.peek(), "⠁" );
	scanner.pop();
	assert_eq!( scanner.cursor(), 6 );
}

#[test]
fn scanner_works_if_the_chat_loves_me() {
//	let mut scanner = Scanner::new( "❤️ ♥️ ❤" );
	let mut scanner = Scanner::new( "❤ " );

	assert_eq!( scanner.peek(), "❤" );
	scanner.pop();
	assert_eq!( scanner.cursor(), 3 );
	assert_eq!( scanner.peek(), " " );
	scanner.pop();
	assert_eq!( scanner.cursor(), 4 );
}

#[test]
fn tokenizer_checks_empty() {
	let scanner = Scanner::new( "" );
	let tokenizer = Tokenizer::new( scanner );

	assert_eq!( tokenizer.empty(), true );
}

#[test]
fn tokenizer_checks_not_empty() {
	let scanner = Scanner::new( "a" );
	let tokenizer = Tokenizer::new( scanner );

	assert_eq!( tokenizer.empty(), false );
}

#[test]
fn tokenizer_tokenizes_number() {
	let scanner = Scanner::new( "123" );
	let mut tokenizer = Tokenizer::new( scanner );

	assert_eq!( tokenizer.next(), Token::OperandI32( 123 ) );
}

/*
// :TODO: implement error handling
#[test]
fn tokenizer_failes_on_large_number() {
	let scanner = Scanner::new( "12345678901234567890123456789012345678901234567890123456789012345678901234567890" );
	let mut tokenizer = Tokenizer::new( scanner );

	assert_eq!( tokenizer.next(), Token::NONE );
}
*/

#[test]
fn tokenizer_tokenizes_numbers_with_whitespace() {
	let scanner = Scanner::new( "123 456" );
	let mut tokenizer = Tokenizer::new( scanner );

	assert_eq!( tokenizer.next(), Token::OperandI32( 123 ) );
	assert_eq!( tokenizer.next(), Token::Whitespace );
	assert_eq!( tokenizer.next(), Token::OperandI32( 456 ) );
}

#[test]
fn tokenizer_tokenizes_numbers_with_leading_whitespace() {
	let scanner = Scanner::new( "      123 456" );
	let mut tokenizer = Tokenizer::new( scanner );

	assert_eq!( tokenizer.next(), Token::Whitespace );
	assert_eq!( tokenizer.next(), Token::OperandI32( 123 ) );
	assert_eq!( tokenizer.next(), Token::Whitespace );
	assert_eq!( tokenizer.next(), Token::OperandI32( 456 ) );
}

#[test]
fn tokenizer_tokenizes_whitespace_with_error() {
	let scanner = Scanner::new( "         #" );
	let mut tokenizer = Tokenizer::new( scanner );

	assert_eq!( tokenizer.next(), Token::Whitespace );
	assert_eq!( tokenizer.next(), Token::ERROR );
	assert_eq!( tokenizer.next(), Token::ERROR );
	assert_eq!( tokenizer.empty(), false );
}

#[test]
fn tokenizer_tokenizes_simple_expression() {
	let scanner = Scanner::new( "123 + 456 * 789" );
	let mut tokenizer = Tokenizer::new( scanner );

	assert_eq!( tokenizer.next(), Token::OperandI32( 123 ) );
	assert_eq!( tokenizer.next(), Token::Whitespace );
//	assert_eq!( tokenizer.next(), Token::Operator( _ ) );
	let t = tokenizer.next();
	dbg!(&t);
	match t {
		Token::Operator( o ) => assert_eq!( o.literal, "+" ),
		_ => panic!("!!!"),
	}
	assert_eq!( tokenizer.next(), Token::Whitespace );
	assert_eq!( tokenizer.next(), Token::OperandI32( 456 ) );
	assert_eq!( tokenizer.next(), Token::Whitespace );
	let t = tokenizer.next();
	dbg!(&t);
	match t {
		Token::Operator( o ) => assert_eq!( o.literal, "*" ),
		_ => panic!("!!!"),
	}
	assert_eq!( tokenizer.next(), Token::Whitespace );
	assert_eq!( tokenizer.next(), Token::OperandI32( 789 ) );
}

#[test]
fn infix_to_postfix_simple() {
	// "1 + 2"
	// -> "1, 2, +"
}

