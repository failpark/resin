use anyhow::Result;

use super::{
	scope::Scope,
	toml::{
		TOMLItemConfig,
		TOML,
	},
	type_::Type,
};

#[derive(Debug, PartialEq, Default)]
pub struct ItemConfig {
	pub items: Vec<String>,
	/// capitalize the DEFAULT values
	capitalize: bool,
	/// If you really don't like a default
	ignore: Vec<String>,
}

impl ItemConfig {
	fn option_vec_helper(vec: Option<Vec<String>>) -> Vec<String> {
		vec.unwrap_or_default()
	}

	fn capitalize_self(&mut self) {
		Self::capitalize(&mut self.items);
	}

	// allocating the same vec again seemed overkill...
	fn capitalize(items: &mut [String]) {
		for item in items.iter_mut() {
			let (first, _last) = item.split_at_mut(1);
			first.make_ascii_uppercase();
		}
	}

	fn merge(&mut self, merge: ItemConfig) {
		self.items.extend(merge.items);
		self.ignore.extend(merge.ignore);
		self.capitalize = self.capitalize || merge.capitalize;
	}

	fn type_default() -> Self {
		Self {
			items: Type::get_vec(),
			..Default::default()
		}
	}

	fn scope_default() -> Self {
		Self {
			items: Scope::get_vec(),
			..Default::default()
		}
	}
}

pub struct Config {
	scopes: ItemConfig,
	types: ItemConfig,
	sign: bool,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			types: ItemConfig::type_default(),
			scopes: ItemConfig::scope_default(),
			sign: false,
		}
	}
}

impl Config {
	pub fn get_signoff(&self) -> bool {
		self.sign
	}

	pub fn get_scope_items(&self) -> &[String] {
		self.scopes.items.as_slice()
	}

	pub fn get_type_items(&self) -> &[String] {
		self.types.items.as_slice()
	}

	pub fn get() -> Result<Self> {
		Ok(Self::get_conf(TOML::get()?))
	}

	fn merge(&mut self, config: Config) {
		self.types.merge(config.types);
		self.scopes.merge(config.scopes);
		self.sign = self.sign || config.sign;
	}

	fn get_conf(toml: Option<TOML>) -> Self {
		if toml.is_none() {
			return Self::default();
		}
		let mut toml: Config = toml.unwrap().into();
		let mut config = Config::default();
		// we only want to capitalize the default values
		if toml.types.capitalize {
			config.types.capitalize_self();
		}
		if toml.scopes.capitalize {
			config.scopes.capitalize_self();
		}

		toml.merge(config);
		toml
	}
}

impl From<TOML> for Config {
	fn from(val: TOML) -> Self {
		Self {
			types: val.types.into(),
			scopes: val.scopes.into(),
			sign: val.sign.unwrap_or_default(),
		}
	}
}

impl From<TOMLItemConfig> for ItemConfig {
	fn from(val: TOMLItemConfig) -> Self {
		Self {
			items: Self::option_vec_helper(val.items),
			capitalize: val.capitalize.unwrap_or_default(),
			ignore: Self::option_vec_helper(val.ignore),
		}
	}
}

impl From<Option<TOMLItemConfig>> for ItemConfig {
	fn from(val: Option<TOMLItemConfig>) -> Self {
		if let Some(val) = val {
			val.into()
		} else {
			Self::default()
		}
	}
}
