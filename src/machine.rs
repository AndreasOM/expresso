
use crate::variable_storage::VariableStorage;

pub struct Machine {
	variable_storage: VariableStorage,
}

impl Machine {
	pub fn new() -> Self {
		Self {
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
}
