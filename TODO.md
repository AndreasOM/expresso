# TODO


## TODO
- [ ] Allow underscores `_` in variable names

- [ ] Decide if char might be better return type for scanner
- [ ] Decide if we want to support multi character operators
- [ ] Add pretty printing for tokens
- [ ] Improve error handling
- [ ] Use builder pattern for Expression
- [ ] Add better error reporting for broken expressions, e.g. by add a lexer to the tokenizer (that is just part of the solution)

## Done
- [x] Create a real expression struct instead of a Vec<Token>
- [x] Decide how to handle I32/F32 combinations
	All calculations that allow parts to be f32 are always handled as f32
- [x] Add validation to expressions
- [x] Add support for literals - or rather variables, and functions
- [x]Add debug to Expression
- [x] Silence "Expanding variable"

