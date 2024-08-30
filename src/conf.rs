use std::env::current_dir;
use std::fs;

use anyhow::Result;
use itertools::Itertools;
use serde::Deserialize;

use crate::utils::{output_failure, output_info, to_string_vec};

#[derive(Debug, PartialEq, Default)]
pub struct ItemConfig {
	pub items: Vec<String>,
	capitalize: bool,
	ignore: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TOMLItemConfig {
	items: Option<Vec<String>>,
	capitalize: Option<bool>,
	ignore: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Force {
	change_types: Option<Vec<String>>,
	scopes: Option<Vec<String>>,
}

pub struct Config {
	pub scopes: ItemConfig,
	pub change_types: ItemConfig,
	pub sign: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
struct RawTOML {
	change_types: Option<TOMLItemConfig>,
	scopes: Option<TOMLItemConfig>,
	sign: Option<bool>,
	force: Option<Force>,
}

#[derive(Deserialize)]
struct OldTOML {
	change_types: Option<Vec<String>>,
	scopes: Option<Vec<String>>,
	sign: Option<bool>,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			change_types: ItemConfig {
				items: to_string_vec(vec!["feat", "fix", "docs", "style", "refactor", "chore"]),
				capitalize: false,
				ignore: vec![],
			},
			scopes: ItemConfig {
				items: to_string_vec(vec![
					"none", "mod", "theme", "block", "style", "doc", "release", "dev",
				]),
				capitalize: false,
				ignore: vec![],
			},
			sign: false,
		}
	}
}

// Reading config file
fn read_toml() -> Result<Option<RawTOML>> {
	let file_name = String::from("resin.toml");
	let current_dir = current_dir()?;
	let ancestors = current_dir.ancestors();
	for ancestor in ancestors {
		let path = ancestor.join(&file_name);
		if path.exists() {
			let content = fs::read_to_string(&path)?;
			let toml: Result<RawTOML, _> = toml::from_str(&content);
			// Jesus christ why is the error type 3 nestings deep?
			if let Ok(toml) = toml {
				return Ok(Some(toml));
			}
			let old_toml = read_old_toml(&content);
			if old_toml.is_some() {
				output_info(&format!("Using old TOML format from: {:?}", path));
				return Ok(old_toml);
			}
			output_failure(&format!("Failed to parse TOML file: {:?}", path));
		}
	}
	Ok(None)
}

fn read_old_toml(content: &str) -> Option<RawTOML> {
	let toml = toml::from_str::<OldTOML>(&content);
	if let Ok(toml) = toml {
		let change_types = toml.change_types.map(|v| v.into_iter().collect());
		let scopes = toml.scopes.map(|v| v.into_iter().collect());
		Some(RawTOML {
			change_types: Some(TOMLItemConfig {
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
			force: None,
		})
	} else {
		None
	}
}

fn capitalize(items: &mut Vec<String>) -> () {
	for item in items.iter_mut() {
		let (first, _last) = item.split_at_mut(1);
		first.make_ascii_uppercase();
	}
}

pub fn read() -> Result<Config> {
	let toml = read_toml()?;
	get_conf(toml)
}

fn get_conf(toml: Option<RawTOML>) -> Result<Config> {
	let mut config = Config::default();
	let mut force = None;
	if let Some(toml) = toml {
		// simply override field
		config.sign = toml.sign.unwrap_or(config.sign);

		if let Some(toml_force) = toml.force {
			force = Some(toml_force);
		}

		if let Some(change_types) = toml.change_types {
			config.change_types.capitalize = change_types
				.capitalize
				.unwrap_or(config.change_types.capitalize);
			config
				.change_types
				.items
				.extend(change_types.items.unwrap_or_default());
			config
				.change_types
				.ignore
				.extend(change_types.ignore.unwrap_or_default());
		}
		if let Some(scopes) = toml.scopes {
			config.scopes.capitalize = scopes.capitalize.unwrap_or(config.scopes.capitalize);
			config.scopes.items.extend(scopes.items.unwrap_or_default());
			config
				.scopes
				.ignore
				.extend(scopes.ignore.unwrap_or_default());
		}
	}

	// Removing duplicates
	config.change_types.items = config.change_types.items.into_iter().unique().collect();
	config.scopes.items = config.scopes.items.into_iter().unique().collect();

	// Remove ignored scopes and change types
	config
		.change_types
		.items
		.retain(|s| !config.change_types.ignore.contains(s));
	config
		.scopes
		.items
		.retain(|s| !config.scopes.ignore.contains(s));

	// Capitalize where needed
	if config.change_types.capitalize {
		capitalize(&mut config.change_types.items);
	}
	if config.scopes.capitalize {
		capitalize(&mut config.scopes.items);
	}

	if let Some(force) = force {
		if let Some(change_types) = force.change_types {
			config.change_types.items = change_types;
		}
		if let Some(scopes) = force.scopes {
			config.scopes.items = scopes;
		}
	}
	Ok(config)
}

#[cfg(test)]
mod test {
	use super::{read_old_toml, RawTOML, TOMLItemConfig};

	#[test]
	fn test_old() {
		let old_toml = "
			scopes = [
				'conf',
				'type',
				'scope',
				'git',
				'validate',
				'ci',
				'deps'
			]
			change_types = ['breaking']
		";
		let config = read_old_toml(old_toml).expect("parse failed");
		let raw_toml = RawTOML {
			change_types: Some(TOMLItemConfig {
				items: Some(vec!["breaking".to_string()]),
				capitalize: None,
				ignore: None,
			}),
			scopes: Some(TOMLItemConfig {
				items: Some(vec![
					"conf".to_string(),
					"type".to_string(),
					"scope".to_string(),
					"git".to_string(),
					"validate".to_string(),
					"ci".to_string(),
					"deps".to_string(),
				]),
				capitalize: None,
				ignore: None,
			}),
			sign: None,
			force: None,
		};
		assert_eq!(config, raw_toml);
	}
}
