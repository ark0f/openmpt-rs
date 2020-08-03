//! Definitions for all types and methods used query module metadata

use super::Module;
use openmpt_sys;

impl Module {
	/// Get a metadata item value.
	///
	/// ### Parameters
	/// * `key` : Metadata item key to query, from the `MetadataKey` enum.
	///
	/// ### Returns
	/// The associated value for key, or None in case of error.
	pub fn get_metadata(&mut self, key : &str) -> Option<String> {
		get_string_with_string! (key, {
			openmpt_sys::openmpt_module_get_metadata(self.inner, key)
		})
	}

	pub fn get_metadata_keys(&mut self) -> String {
		let opt_string = get_string! {
			openmpt_sys::openmpt_module_get_metadata_keys(self.inner)
		};

		opt_string.expect("Got null pointer instead of string")
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use super::super::test_helper;

	#[test]
	fn dummy_file_is_xm() {
		let mut module = test_helper::load_file_as_module("empty_module.xm").unwrap();
		assert_eq!(module.get_metadata("type").unwrap(), "xm");
		assert_eq!(module.get_metadata("type_long").unwrap(), "FastTracker II");
	}
}
