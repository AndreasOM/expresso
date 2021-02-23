
use core::fmt::Debug;
use core::fmt::Formatter;

#[derive(Clone,PartialEq,Eq)]
pub struct Operator {
	pub literal: &'static str,
	pub precendence: i8,
	pub is_right_associative: bool,
}


impl Debug for Operator {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		formatter.write_fmt(format_args!("{}, {}, {}", self.literal, self.precendence, if self.is_right_associative { "right" } else { "left" } ))
	}
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

