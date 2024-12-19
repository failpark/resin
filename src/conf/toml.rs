use std::{
	env::current_dir,
	fs,
	path::{
		Path,
		PathBuf,
	},
};

use serde::Deserialize;

use super::old;
use crate::utils::{
	output_failure,
	output_info,
};

static FILENAME: &str = "resin.toml";

#[derive(Debug, Deserialize, PartialEq)]
pub struct TOMLItemConfig {
	pub items: Option<Vec<String>>,
	// At work I was outvoted and I had to include this
	pub capitalize: Option<bool>,
	pub ignore: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq)]
// somehow writing Toml feels wrong
// I mean its an acronym so either all upper or all lowercase
#[allow(clippy::upper_case_acronyms)]
pub struct TOML {
	// there was a field `force` here ...
	// I think we can just expect every change_type to be written as wanted
	pub types: Option<TOMLItemConfig>,
	pub scopes: Option<TOMLItemConfig>,
	pub sign: Option<bool>,
}

impl TOML {
	fn read(path: &PathBuf) -> anyhow::Result<Self> {
		let content = fs::read_to_string(path)?;
		// we have to call `?` to convert from toml::de::Error
		// to anyhow::Error
		// and we have to do that because we have the following Error types in this function:
		// - std::io::Error
		// - toml::de::Error
		let toml = toml::from_str(&content);
		if let Ok(toml) = toml {
			return Ok(toml);
		}

		// i could just pass &content here to avoid re-reading the file
		// but users of the old format need to be punished
		// (also I'm probably the only one using this anymore sooooooooo ¯\_(ツ)_/¯)
		let old = old::TOML::read(path);
		// do this whole song and dance to use the Err from the preferred toml
		if let Ok(old) = old {
			// let the user know they're commiting crimes
			old::TOML::print_info(path);
			Ok(old.into())
		} else {
			Self::print_err(path);
			Err(
				toml
					.err()
					// we can unwrap since we know that its only possible to have an err here
					// (otherwise we would have returned from the new TOML read)
					.unwrap()
					.into(),
			)
		}
	}

	fn print_err(path: &Path) {
		let path = path.display();
		output_failure(format!("Failed to parse TOML file: {path}").as_str());
		output_info("Falling back to defaults");
	}

	/// Reading config file
	pub fn get() -> anyhow::Result<Option<Self>> {
		let current_dir = current_dir()?;
		let ancestors = current_dir.ancestors();
		for ancestor in ancestors {
			let path = ancestor.join(FILENAME);
			if path.exists() {
				return Ok(Some(Self::read(&path)?));
			}
		}
		// if no config was found just use defaults
		Ok(None)
	}
}
