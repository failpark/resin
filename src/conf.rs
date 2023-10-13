use std::env::current_dir;
use std::fs;

use anyhow::Result;
use itertools::Itertools;
use serde::Deserialize;

use crate::utils::to_string_vec;

#[derive(Debug, Deserialize)]
struct RawTOML {
	scopes: Option<Vec<String>>,
	change_types: Option<Vec<String>>,
	sign: Option<bool>,
}

#[derive(Debug, PartialEq)]
pub struct Config {
	pub scopes: Vec<String>,
	pub change_types: Vec<String>,
	pub sign: bool,
}

pub fn read() -> Result<Config> {
	let mut config = Config {
		scopes: to_string_vec(vec![
			"none", "mod", "theme", "block", "style", "lint", "doc", "release", "dev",
		]),
		change_types: to_string_vec(vec![
			"Feat", "Fix", "Docs", "Style", "Refactor", "Perf", "Test", "Build", "CI", "Chore",
			"Revert",
		]),
		sign: false,
	};

	// Reading local config file
	let file_name = String::from("resin.toml");
	let current_dir = current_dir()?;
	let ancestors = current_dir.ancestors();
	for ancestor in ancestors {
		let path = ancestor.join(&file_name);
		if path.exists() {
			let content = fs::read_to_string(path)?;
			let raw_data: RawTOML = toml::from_str(&content)?;
			config.scopes.extend(
				raw_data
					.scopes
					.unwrap_or_default()
					.iter()
					.map(|s| s.to_lowercase()),
			);
			config
				.change_types
				.extend(raw_data.change_types.unwrap_or_default().iter().map(|s| {
					let mut c = s.chars();
					match c.next() {
						None => String::new(),
						Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
					}
				}));
			if raw_data.sign.is_some() {
				config.sign = raw_data.sign.unwrap();
			}
			break;
		}
	}

	// Removing duplicates
	config.scopes = config.scopes.into_iter().unique().collect();

	Ok(config)
}

#[cfg(test)]
mod tests {
	use super::*;
	use anyhow::Result;

	use crate::utils::to_string_vec;

	#[test]
	fn test_read() -> Result<()> {
		assert_eq!(
			read()?,
			Config {
				scopes: to_string_vec(vec![
					"none", "mod", "theme", "block", "style", "lint", "doc", "release", "dev",
					"conf", "type", "scope", "git", "validate", "ci",
				]),
				change_types: to_string_vec(vec![
					"Feat", "Fix", "Docs", "Style", "Refactor", "Perf", "Test", "Build", "CI",
					"Chore", "Revert", "Breaking",
				]),
				sign: false,
			},
		);
		Ok(())
	}
}
