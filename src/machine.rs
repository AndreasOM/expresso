
use crate::function_table::FunctionTable;
use crate::variable_stack::VariableStack;
use crate::variable_storage::VariableStorage;
use crate::variables::Variable;

pub struct Machine {
	function_table: FunctionTable,
	variable_storage: VariableStorage,
}

impl Machine {
	pub fn new() -> Self {
		Self {
			function_table: FunctionTable::new(),
			variable_storage: VariableStorage::new(),
		}
	}

	pub fn load_variable_storage( &mut self, filename: &str ) -> Result< (), Box< dyn std::error::Error > >{
		match VariableStorage::load( filename ) {
			Ok( vs ) => {
				self.variable_storage = vs;
				Ok(())				
			},
			Err( e ) => {
				Err( e )
			},
		}
	}

	pub fn save_variable_storage( &mut self, filename: &str ) -> Result< (), Box< dyn std::error::Error > >{
		match self.variable_storage.save( filename ) {
			Ok( _ ) => {
				Ok(())				
			},
			Err( e ) => {
				Err( e )
			},
		}
	}

	pub fn get_mut_variable_storage( &mut self ) -> &mut VariableStorage {
		&mut self.variable_storage
	}

	pub fn get_mut_function_table( &mut self ) -> &mut FunctionTable {
		&mut self.function_table
	}
	pub fn get_function_table( &self ) -> &FunctionTable {
		&self.function_table
	}

	pub fn call_function( &mut self, name: &str, argc: u16, stack: &mut VariableStack ) -> bool {
		if let Some( f ) = self.function_table.find( name ) {
			f( argc, stack, &mut self.variable_storage )
		} else {
			println!("Function not found for CallFunction {} with {} arguments", name, argc );
			// :HACK:
			for _ in 0..argc {
				stack.pop();	// function would take it's arguments from stack
			};
			// and return a result

			stack.push( Variable::I32( argc as i32 ) );
			false
		}		
	}
}
