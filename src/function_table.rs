
use std::collections::HashMap;

use crate::variable_stack::VariableStack;
use crate::variable_storage::VariableStorage;

pub struct FunctionTable {
	functions: HashMap<String, Box< dyn Fn( u16, &mut VariableStack, &mut VariableStorage ) -> bool > >,
}

impl FunctionTable {
	pub fn new() -> Self {
		Self {
			functions: HashMap::new(),
		}
	}

	pub fn register<F: 'static>( &mut self, name: &str, f: F )
		where F: Fn( u16, &mut VariableStack, &mut VariableStorage ) -> bool
	{
		self.functions.insert(
			name.to_string(),
			Box::new( f )
		);
	}

	pub fn find( &self, name: &str ) -> Option< &Box<dyn for<'r, 's> Fn(u16, &'r mut VariableStack, &'s mut VariableStorage) -> bool> > {
		self.functions.get( name )
	}
}

impl core::fmt::Debug for FunctionTable {
	fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		formatter.write_fmt( format_args!("FunctionTable :TODO:") )
	}
}

