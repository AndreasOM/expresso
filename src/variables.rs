
use serde::{Deserialize,Serialize};
//use serde_yaml;

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub enum Variable {
	I32( i32 ),
	F32( f32 ),
	String( String ),
	EMPTY,
	ERROR( String ),
}

