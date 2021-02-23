
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Operator {
	pub literal: &'static str,
	pub precendence: i8,
	pub is_right_associative: bool,
}

pub const OPERATORS: [Operator;4] = [
	Operator {
		literal: "*",
		precendence: 5,
		is_right_associative: false,
	},
	Operator {
		literal: "/",
		precendence: 5,
		is_right_associative: false,
	},
	Operator {
		literal: "+",
		precendence: 6,
		is_right_associative: false,
	},
	Operator {
		literal: "-",
		precendence: 6,
		is_right_associative: false,
	},
];

