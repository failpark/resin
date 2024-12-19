use std::{
	fmt::Display,
	process::exit,
};

use colored::Colorize;

pub fn output_success(message: &str) {
	println!("{} {}", "✔".green(), message.bold())
}

pub fn output_info(message: &str) {
	println!("{} {}", "ℹ".blue(), message.bold())
}

pub fn output_failure(message: &str) {
	println!("{} {}", "✗".red(), message.bold());
}

pub fn print_fail<T>(do_task: T)
where
	T: Display,
{
	output_failure(
		format!("Failed to {do_task}. Try running the command manually and resolving the error",)
			.as_str(),
	);
}

pub fn fail<T>(error: T)
where
	T: Display,
{
	print_fail(error);
	exit(1);
}

pub fn parse_jira(name: &str) -> Option<String> {
	let ticket_regex = regex::Regex::new("([A-Za-z_]{3,}-[0-9]+)").unwrap();
	ticket_regex
		.find(name)
		.map(|hit| String::from(hit.as_str()))
}

#[cfg(test)]
mod tests {
	use pretty_assertions::assert_eq;

	use super::*;

	#[test]
	fn test_parse_jira() {
		assert_eq!(parse_jira("Test-123"), Some("Test-123".into()));
		assert_eq!(parse_jira("main"), None);
	}
}
