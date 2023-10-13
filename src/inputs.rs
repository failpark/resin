use std::process::Command;

use anyhow::{Context, Result};
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, FuzzySelect, Input};

use crate::conf;

pub struct Inputs {
	pub change_type: String,
	pub scope: String,
	pub description: String,
	pub long_description: String,
	pub breaking_changes: String,
	pub ticket: String,
}

pub fn get_inputs(config: &conf::Config) -> Result<Inputs> {
	let theme = ColorfulTheme::default();

	let change_type_selection = FuzzySelect::with_theme(&theme)
		.with_prompt("Type")
		.default(0)
		.items(&config.change_types)
		.interact_opt()
		.context("Failed to present change type selection to user")?
		.unwrap_or_else(|| std::process::exit(1));
	let scope_selection = FuzzySelect::with_theme(&theme)
		.with_prompt("Scope")
		.default(0)
		.items(&config.scopes)
		.interact_opt()
		.context("Failed to present scope selection to user")?
		.unwrap_or_else(|| std::process::exit(1));
	let description: String = Input::with_theme(&theme)
		.with_prompt("Description")
		.validate_with({
			let mut force = None;
			// type + `: `
			let change_type_len =
				String::from(&config.change_types[change_type_selection]).len() + 2;
			// scope + `()`
			let scope_len = String::from(config.scopes.get(scope_selection).unwrap()).len() + 2;
			let max_input_length = if scope_selection != 0 {
				50 - change_type_len - scope_len
			} else {
				50 - change_type_len
			};
			move |input: &String| -> Result<(), String> {
				let input_len = input.len();
				if (input_len) <= max_input_length
					|| force.as_ref().map_or(false, |old| old == input)
				{
					Ok(())
				} else {
					force = Some(input.clone());
					Err(format!(
						"Your can only write {max_input_length} chars and you wrote: {input_len}; \
						 type the same value again to force use"
					))
				}
			}
		})
		.interact_text()
		.context("Failed to ask for description")?;
	let long_description: bool = Confirm::with_theme(&theme)
		.default(false)
		.with_prompt("Longer description (optional)")
		.wait_for_newline(true)
		.interact()
		.context("Failed to ask for longer description")?;

	let long_description = if long_description {
		let template = include_str!("long_desc.template");
		edit::edit(template)?
	} else {
		String::new()
	};
	let long_description = long_description
		.lines()
		.filter(move |line| !line.starts_with('#'))
		.fold(String::new(), |s, l| s + l + "\n");

	let breaking_changes: String = String::new();
	let ticket: String = Input::with_theme(&theme)
		.allow_empty(true)
		.with_initial_text(parse_branch_info())
		.with_prompt("Ticket (optional)")
		.interact_text()
		.context("Failed to ask for ticket")?;
	Ok(Inputs {
		change_type: String::from(&config.change_types[change_type_selection]),
		scope: config.scopes.get(scope_selection).unwrap().to_owned(),
		description,
		long_description,
		breaking_changes,
		ticket,
	})
}

pub fn parse_branch_info() -> String {
	let ticket_regex = regex::Regex::new("([A-Za-z_]{3,}-[0-9]+)").unwrap();
	let branch = Command::new("git")
		.args(["symbolic-ref", "--short", "HEAD"])
		.output();
	let branch = if let Ok(result) = branch {
		String::from_utf8(result.stdout).unwrap_or_default()
	} else {
		String::new()
	};

	if !branch.is_empty() {
		match ticket_regex.find(branch.as_str()) {
			Some(hit) => String::from(hit.as_str()),
			None => String::new(),
		}
	} else {
		String::new()
	}
}
