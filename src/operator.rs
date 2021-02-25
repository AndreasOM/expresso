
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

pub const OPERATOR_MULTIPLY:	Operator = Operator { literal: "*", precendence: 5, is_right_associative: false, };
pub const OPERATOR_DIVIDE:	 	Operator = Operator { literal: "/", precendence: 5, is_right_associative: false, };
pub const OPERATOR_ADD: 	 	Operator = Operator { literal: "+", precendence: 6, is_right_associative: false, };
pub const OPERATOR_SUBTRACT: 	Operator = Operator { literal: "-", precendence: 6, is_right_associative: false, };

pub const OPERATORS: [Operator;4] = [
	OPERATOR_MULTIPLY,
	OPERATOR_DIVIDE,
	OPERATOR_ADD,
	OPERATOR_SUBTRACT,
];

