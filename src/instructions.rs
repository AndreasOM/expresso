
use crate::operator::{Operator,OPERATORS};

#[derive(Debug,PartialEq)]
pub enum Instruction {
	PushI32( i32 ),
	PushF32( f32 ),
	PushVariable( String ),
	PushString( String ),
	Operator( Operator ),
	CallFunction,
	StartList,
	EndList,
	EOF
}
