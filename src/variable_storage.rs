use std::collections::HashMap;

use std::fs::File;
use std::io::prelude::*;

use serde::{Deserialize,Serialize};
use serde_yaml;

#[derive(Debug,Deserialize,Serialize)]
pub enum Variable {
	I32( i32 )
}

#[derive(Debug,Deserialize,Serialize)]
pub struct VariableStorage {
	variables: HashMap<String, Variable>,
}

impl VariableStorage {
	pub fn new() -> Self {
		Self {
			variables: HashMap::new(),
		}
	}

	pub fn load( filename: &str ) -> Result< VariableStorage, Box< dyn std::error::Error > > {
		let f = std::fs::File::open( filename )?;

		let vs: VariableStorage = serde_yaml::from_reader( f )?;

		Ok( vs )
	}

	pub fn save( &self, filename: &str ) -> Result< (), Box< dyn std::error::Error > > {
		let serialized = serde_yaml::to_string(&self).unwrap();
//		dbg!(&serialized);
		let mut f = std::fs::File::create( filename )?;
		f.write_all( &serialized.as_bytes() )?;
		Ok(())
	}

	pub fn get( &self, name: &str ) -> Option< &Variable > {
		self.variables.get( name )
	}

	pub fn set_i32( &mut self, name: &str, value: i32 ) {
		self.variables.insert( name.to_string(), Variable::I32( value ) );
	}
}
