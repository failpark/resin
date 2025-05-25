use std::{
	path::{
		Path,
		PathBuf,
	},
	sync::OnceLock,
};

use anyhow::{
	Context,
	Result,
};
use dialoguer::{
	theme::ColorfulTheme,
	BasicHistory,
	Confirm,
	FuzzySelect,
	Input,
};
use git2::Repository;
use sys_locale::get_locale;

use crate::{
	conf,
	git::{
		get_branch_name,
		get_repo,
	},
	utils::{
		fail,
		parse_jira,
	},
};

pub struct Inputs<'a> {
	pub change_type: &'a str,
	pub scope: Option<String>,
	pub description: String,
	pub long_description: String,
	pub breaking_changes: String,
	pub ticket: String,
}

static THEME: OnceLock<ColorfulTheme> = OnceLock::new();

fn get_theme() -> &'static ColorfulTheme {
	THEME.get_or_init(ColorfulTheme::default)
}

pub fn get_inputs(config: &conf::Config) -> Result<Inputs<'_>> {
	let theme = get_theme();
	let repo = get_repo();
	// before doing anything check index
	super::git::check_emptiness(&repo);

	let type_selection = ask_for_change(theme, config.get_type_items())?;
	// I think panicing here is ok
	let type_: &str = config.get_type_items().get(type_selection).unwrap();
	let scope = ask_for_scope(theme)?;
	let scope = if scope {
		let scope = get_for_scope(theme)?;
		let len = scope.len();
		(Some(scope), len)
	} else {
		(None, 0)
	};
	let description: String = ask_for_desc(theme, calc_max_len(&type_.len(), &scope.1))?;
	let long_description: bool = ask_for_long_desc(theme)?;

	let long_description = if long_description {
		get_long_desc()?
	} else {
		String::new()
	};

	let breaking_changes: String = ask_for_breaking_changes(theme)?;
	let ticket: String = ask_for_ticket(theme, &repo)?;
	Ok(Inputs {
		change_type: type_,
		scope: scope.0,
		description,
		long_description,
		breaking_changes,
		ticket,
	})
}

fn ask_for_change(theme: &ColorfulTheme, items: &[String]) -> Result<usize, anyhow::Error> {
	FuzzySelect::with_theme(theme)
		.with_prompt("Type")
		.default(0)
		.items(items)
		.interact()
		.context("Failed to present change type selection to user")
}

fn ask_for_scope(theme: &ColorfulTheme) -> Result<bool, anyhow::Error> {
	Confirm::with_theme(theme)
		.default(false)
		.with_prompt("Scope (optional)")
		.wait_for_newline(true)
		.interact()
		.context("Failed to ask for longer description")
}

fn get_for_scope(theme: &ColorfulTheme) -> Result<String, anyhow::Error> {
	Input::with_theme(theme)
		.with_prompt("Scope")
		.interact_text()
		.context("Failed to present scope selection to user")
}

fn calc_max_len(change_type_len: &usize, scope_len: &usize) -> usize {
	// type + `: `
	let change_type_len = change_type_len + 2;
	let scope_len = if scope_len == &0 {
		0
	} else {
		// scope + `()`
		scope_len + 2
	};
	// header is only supposed to be 50 chars long
	50 - change_type_len - scope_len
}

fn ask_for_desc(theme: &ColorfulTheme, max_input_length: usize) -> Result<String, anyhow::Error> {
	let mut history = BasicHistory::new().max_entries(4).no_duplicates(true);

	Input::with_theme(theme)
		.with_prompt("Description")
		.history_with(&mut history)
		.validate_with({
			let mut force = None;

			move |input: &String| -> Result<(), String> {
				let input_len = input.len();
				if (input_len) <= max_input_length || (force.as_ref() == Some(input)) {
					Ok(())
				} else {
					force = Some(input.clone());
					Err(format!(
						"Your can only write {max_input_length} chars and you wrote: {input_len}; type the \
						 same value again to force use"
					))
				}
			}
		})
		.interact_text()
		.context("Failed to ask for description")
}

fn get_long_desc() -> Result<String, anyhow::Error> {
	let locale = get_locale().unwrap_or_else(|| String::from("en-US"));
	let template = match locale.as_str() {
		"de-DE" => include_str!("templates/de.template"),
		_ => include_str!("templates/en.template"),
	};
	let long_description = edit::edit(template)?;
	let long_description = long_description
		.lines()
		.filter(move |line| !line.starts_with('#'))
		.fold(String::new(), |s, l| s + l + "\n");
	Ok(long_description)
}

fn ask_for_long_desc(theme: &ColorfulTheme) -> Result<bool, anyhow::Error> {
	Confirm::with_theme(theme)
		.default(false)
		.with_prompt("Longer description (optional)")
		.wait_for_newline(true)
		.interact()
		.context("Failed to ask for longer description")
}

fn ask_for_breaking_changes(theme: &ColorfulTheme) -> Result<String, anyhow::Error> {
	Input::with_theme(theme)
		.allow_empty(true)
		.with_prompt("Breaking change (optional)")
		.interact_text()
		.context("Failed to ask for breaking changes")
}

fn ask_for_ticket(theme: &ColorfulTheme, repo: &Repository) -> Result<String, anyhow::Error> {
	// there has to be a better solution...
	let init = if let Some(name) = get_branch_name(repo) {
		parse_jira(&name).unwrap_or_default()
	} else {
		String::new()
	};
	Input::with_theme(theme)
		.allow_empty(true)
		.with_initial_text(init)
		.with_prompt("Ticket (optional)")
		.interact_text()
		.context("Failed to ask for ticket")
}

pub fn ask_for_path() -> bool {
	let path = Confirm::with_theme(get_theme())
		.default(false)
		.with_prompt("Specify path?")
		.wait_for_newline(true)
		.interact();
	if path.is_err() {
		fail("Failed to ask for prompt");
	}
	path.unwrap()
}

pub fn prompt_for_path() -> PathBuf {
	let path: Result<String, _> = Input::with_theme(get_theme())
		.with_prompt("Path")
		.interact_text();
	if path.is_err() {
		fail("Failed to prompt for path");
	}
	Path::new(path.unwrap().as_str()).to_path_buf()
}
