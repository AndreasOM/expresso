

use super::converter::Converter;
use super::expression::Expression;
use super::operator::*;
use super::scanner::Scanner;
use super::token_stack::TokenStack;
use super::tokenizer::{Token, Tokenizer};

use std::mem;

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

#[test]
fn tokenizer_tokenizes_float() {
	let scanner = Scanner::new( "12.34" );
	let mut tokenizer = Tokenizer::new( scanner );

	assert_eq!( tokenizer.next(), Token::OperandF32( 12.34 ) );

	let scanner = Scanner::new( "1234.3456" );
	let mut tokenizer = Tokenizer::new( scanner );

	assert_eq!( tokenizer.next(), Token::OperandF32( 1234.3456 ) );
	let scanner = Scanner::new( "987654321.123456789" );
	let mut tokenizer = Tokenizer::new( scanner );

	assert_eq!( tokenizer.next(), Token::OperandF32( 987654321.123456789 ) );
}

#[test]
fn tokenizer_tokenizes_braces() {
	let scanner = Scanner::new( "()" );
	let mut tokenizer = Tokenizer::new( scanner );

	assert_eq!( tokenizer.next(), Token::BraceLeft );
	assert_eq!( tokenizer.next(), Token::BraceRight );

	assert_eq!( tokenizer.empty(), true );
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
	assert_eq!( tokenizer.next(), Token::EOF );
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
	assert_eq!( mem::discriminant( &tokenizer.next() ), mem::discriminant( &Token::ERROR( "" ) ) );
//	assert_eq!( tokenizer.next(), Token::ERROR( _ ) );
	assert_eq!( mem::discriminant( &tokenizer.next() ), mem::discriminant( &Token::ERROR( "" ) ) );
//	assert_eq!( tokenizer.next(), Token::ERROR );
	assert_eq!( tokenizer.empty(), false );
}

#[test]
fn tokenizer_tokenizes_simple_expression() {
	let scanner = Scanner::new( "123 + 456 * 789 - 222 / 333" );
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
	assert_eq!( tokenizer.next(), Token::Whitespace );

	let t = tokenizer.next();
	dbg!(&t);
	match t {
		Token::Operator( o ) => assert_eq!( o.literal, "-" ),
		_ => panic!("!!! {:?}", &t ),
	}
	assert_eq!( tokenizer.next(), Token::Whitespace );
	assert_eq!( tokenizer.next(), Token::OperandI32( 222 ) );
	assert_eq!( tokenizer.next(), Token::Whitespace );

	let t = tokenizer.next();
	dbg!(&t);
	match t {
		Token::Operator( o ) => assert_eq!( o.literal, "/" ),
		_ => panic!("!!!"),
	}
	assert_eq!( tokenizer.next(), Token::Whitespace );
	assert_eq!( tokenizer.next(), Token::OperandI32( 333 ) );

	assert_eq!( tokenizer.empty(), true );
}

#[test]
fn infix_to_postfix_simple() {
	// "1 + 2"
	// -> "1, 2, +"

	let mut converter = Converter::new( "1 + 2" );
	let postfix = converter.to_postfix( );

	dbg!( &postfix );
	let mut iter = postfix.iter();

	assert_eq!( iter.next(), Some( &Token::OperandI32( 1 ) ) );
	assert_eq!( iter.next(), Some( &Token::OperandI32( 2 ) ) );
	assert_eq!( iter.next(), Some( &Token::Operator( OPERATOR_ADD ) ) );
	assert_eq!( iter.next(), None );
}

#[test]
fn infix_to_postfix_simple_mixed_numbers() {
	// "1 + 2"
	// -> "1, 2, +"

	let mut converter = Converter::new( "1 + 2.3" );
	let postfix = converter.to_postfix( );

	dbg!( &postfix );
	let mut iter = postfix.iter();

	assert_eq!( iter.next(), Some( &Token::OperandI32( 1 ) ) );
	assert_eq!( iter.next(), Some( &Token::OperandF32( 2.3 ) ) );
	assert_eq!( iter.next(), Some( &Token::Operator( OPERATOR_ADD ) ) );
	assert_eq!( iter.next(), None );
}

#[test]
fn infix_to_postfix_simple_braces() {
	// "( 1 + 2 ) * 3"
	// -> "1, 2, +, 3, *"

	let mut converter = Converter::new( "( 1 + 2 ) * 3" );
	let postfix = converter.to_postfix( );

	dbg!( &postfix );
	let mut iter = postfix.iter();

	assert_eq!( iter.next(), Some( &Token::OperandI32( 1 ) ) );
	assert_eq!( iter.next(), Some( &Token::OperandI32( 2 ) ) );
	assert_eq!( iter.next(), Some( &Token::Operator( OPERATOR_ADD ) ) );
	assert_eq!( iter.next(), Some( &Token::OperandI32( 3 ) ) );
	assert_eq!( iter.next(), Some( &Token::Operator( OPERATOR_MULTIPLY ) ) );
	assert_eq!( iter.next(), None );
}

#[test]
fn infix_to_postfix_complex() {
	// "1 + 2 + 3 - 4 * 5"
	// -> "1, 2, +, 3, +, 4, 5, *, -"

	let mut converter = Converter::new( "1 + 2 + 3 - 4 * 5" );
	let postfix = converter.to_postfix( );

	dbg!( &postfix );
	let mut iter = postfix.iter();

	assert_eq!( iter.next(), Some( &Token::OperandI32( 1 ) ) );
	assert_eq!( iter.next(), Some( &Token::OperandI32( 2 ) ) );
	assert_eq!( iter.next(), Some( &Token::Operator( OPERATOR_ADD ) ) );
	assert_eq!( iter.next(), Some( &Token::OperandI32( 3 ) ) );
	assert_eq!( iter.next(), Some( &Token::Operator( OPERATOR_ADD ) ) );
	assert_eq!( iter.next(), Some( &Token::OperandI32( 4 ) ) );
	assert_eq!( iter.next(), Some( &Token::OperandI32( 5 ) ) );
	assert_eq!( iter.next(), Some( &Token::Operator( OPERATOR_MULTIPLY ) ) );
	assert_eq!( iter.next(), Some( &Token::Operator( OPERATOR_SUBTRACT ) ) );
	assert_eq!( iter.next(), None );
}

#[test]
fn expression_works() {
	let mut expression = Expression::new();
	expression.from_str( "1+2" );
	assert_eq!( expression.result_as_i32_or( 0 ), 3i32 );
}

#[test]
fn expression_works_complex() {
	let mut expression = Expression::new();
	expression.from_str( "(1+2*5+9-10)/(4-2)" );
	assert_eq!( expression.result_as_i32_or( 0 ), 5i32 );
}

#[test]
fn expression_returns_correct_default() {
	let mut expression = Expression::new();
	expression.from_str( "" );
	assert_eq!( expression.result_as_i32_or( 42 ), 42i32 );
}

#[test]
fn token_stack_works() {
	let mut token_stack = TokenStack::new();
	assert_eq!( token_stack.empty(), true );

	token_stack.push( Token::OperandI32( 0 ) );
	assert_eq!( token_stack.empty(), false );
	assert_eq!( token_stack.len(), 1 );
	let t = token_stack.pop();
	assert_eq!( t, Some( Token::OperandI32( 0 ) ) );
	assert_eq!( token_stack.empty(), true );
	assert_eq!( token_stack.len(), 0 );
	let t = token_stack.pop();
	assert_eq!( t, None );
	assert_eq!( token_stack.empty(), true );
	assert_eq!( token_stack.len(), 0 );
}

#[test]
fn token_stack_supports_typed_pop() {
	let mut token_stack = TokenStack::new();

	token_stack.push( Token::OperandI32( 1 ) );
	assert_eq!( token_stack.pop_as_f32(), 1.0 );
	assert_eq!( token_stack.empty(), true );

	token_stack.push( Token::OperandF32( 2.0 ) );
	assert_eq!( token_stack.pop_as_f32(), 2.0 );
	assert_eq!( token_stack.empty(), true );

	token_stack.push( Token::OperandI32( 3 ) );
	assert_eq!( token_stack.pop_as_i32(), 3 );
	assert_eq!( token_stack.empty(), true );

	token_stack.push( Token::OperandF32( 4.0 ) );
	assert_eq!( token_stack.pop_as_i32(), 4 );
	assert_eq!( token_stack.empty(), true );
}

#[test]
#[should_panic]
fn token_stack_panics_on_wrong_type() {
	let mut token_stack = TokenStack::new();

	token_stack.push( Token::Operator( OPERATOR_ADD ) );
	assert_eq!( token_stack.pop_as_i32(), 4 );
	assert_eq!( token_stack.empty(), true );	
}
