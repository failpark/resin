use anyhow::{Context, Result};
use dialoguer::theme::ColorfulTheme;
use dialoguer::{FuzzySelect, Input};

use crate::conf;

pub struct Inputs {
	pub change_type: String,
	pub scope: String,
	pub description: String,
	pub long_description: String,
	pub breaking_changes: String,
}

pub fn get_inputs(_config: &conf::Config) -> Result<Inputs> {
	let theme = ColorfulTheme::default();
	let change_types = &[
		"Feat", "Fix", "Docs", "Style", "Refactor", "Perf", "Test", "Build", "CI", "Chore",
		"Revert",
	];

	let change_type_selection = FuzzySelect::with_theme(&theme)
		.with_prompt("Type")
		.default(0)
		.items(change_types)
		.interact_opt()
		.context("Failed to present change type selection to user")?
		.unwrap_or_else(|| std::process::exit(1));
	// let scope_selection = FuzzySelect::with_theme(&theme)
		// .with_prompt("Scope")
		// .default(0)
		// .items(&config.scopes)
		// .interact_opt()
		// .context("Failed to present scope selection to user")?
		// .unwrap_or_else(|| std::process::exit(1));
	let description: String = Input::with_theme(&theme)
		.with_prompt("Description")
		.validate_with({
			let mut force = None;
			move |input: &String| -> Result<(), String> {
				let change_type_len = String::from(change_types[change_type_selection]).len();
				let input_len = input.len();
				if (input_len + change_type_len ) <= 50 || force.as_ref().map_or(false, |old| old == input) {
					Ok(())
				} else {
					let max_input_length = 48 - change_type_len;
					force = Some(input.clone());
					Err(format!("Your can only write {max_input_length} chars and you wrote: {input_len}; type the same value again to force use"))
				} 
			}
		})
		.interact_text()
		.context("Failed to ask for description")?;
	let long_description: String = Input::with_theme(&theme)
		.allow_empty(true)
		.with_prompt("Longer description (optional)")
		.interact_text()
		.context("Failed to ask for longer description")?;
	let breaking_changes: String = Input::with_theme(&theme)
		.allow_empty(true)
		.with_prompt("Breaking change (optional)")
		.interact_text()
		.context("Failed to ask for breaking changes")?;
	Ok(Inputs {
		change_type: String::from(change_types[change_type_selection]),
		scope: "none".to_owned(),
		description,
		long_description,
		breaking_changes,
	})
}
