

use super::scanner::Scanner;

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
/*
#[test]
fn scanner_works_with_braille() {
	let mut scanner = Scanner::new( "⡌⠁⠧⠑ ⠼⠁⠒  ⡍⠜⠇⠑⠹⠰⠎ ⡣⠕⠌" );

	assert_eq!( scanner.peek(), "⡌" );

}
*/

