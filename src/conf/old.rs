use std::{
	fs::read_to_string,
	path::{
		Path,
		PathBuf,
	},
};

use serde::Deserialize;

use super::toml::{
	TOMLItemConfig,
	TOML as NewTOML,
};
use crate::utils::output_info;

#[derive(Deserialize)]
/// Old Format
// Toml just looks wrong somehow...
#[allow(clippy::upper_case_acronyms)]
pub struct TOML {
	change_types: Option<Vec<String>>,
	scopes: Option<Vec<String>>,
	sign: Option<bool>,
}

impl TOML {
	pub fn read(path: &PathBuf) -> anyhow::Result<Self> {
		let content = read_to_string(path)?;
		Ok(toml::from_str(&content)?)
	}

	pub fn print_info(path: &Path) {
		let path = path.display();
		let err_msg = format!("Using old TOML format from: {path}");
		output_info(err_msg.as_str());
	}
}

// This is defined here because this will be redundant
// when I remove old::TOML
impl From<TOML> for NewTOML {
	fn from(toml: TOML) -> Self {
		let change_types = toml.change_types.map(|v| v.into_iter().collect());
		let scopes = toml.scopes.map(|v| v.into_iter().collect());
		Self {
			types: Some(TOMLItemConfig {
				items: change_types,
				capitalize: None,
				ignore: None,
			}),
			scopes: Some(TOMLItemConfig {
				items: scopes,
				capitalize: None,
				ignore: None,
			}),
			sign: toml.sign,
		}
	}
}
