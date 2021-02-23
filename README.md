# expresso
Experimental/educational expression parser, and runner


Note: Expresso is a word play on Espresso, and Expression

- [ ] Convert infix to postfix
	- [ ] Create basic scanner
	- [ ] Create a tokenizer
	- [ ] Create a parser
- [ ] Execute postfix code


# Supported
- [x] Whitespace
- [x] Operands
	- [x] i32
	- [x] f32 (Note: No leading or trailing dot! `0.34` and `12.0` work, `.34` and `12.` don't!)

- [x] Operators
	- [x] `+`
	- [x] `*`
	- [x] `-`
	- [x] `/`

- [x] Braces
	- [x] `(`
	- [x] `)`

## Expresso

There is a `expresso` binary included that can be used for quick checking/running of expressions.

# Future
- [ ] Variables
- [ ] Functions (aka "user defined callbacks")


# Other
Development will be done test driven.
