use colored::Colorize;

pub fn to_string_vec(vec: Vec<&str>) -> Vec<String> {
	vec.into_iter().map(String::from).collect()
}

pub fn output_success(message: &str) {
	println!("{} {}", "✔".green(), message.bold())
}

pub fn output_info(message: &str) {
	println!("{} {}", "ℹ".blue(), message.bold())
}

pub fn output_failure(message: &str) {
	println!("{} {}", "✗".red(), message.bold())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_to_string_vec() {
		assert_eq!(to_string_vec(vec!["foo"]), vec![String::from("foo")]);
		assert_eq!(
			to_string_vec(vec!["foo", "bar"]),
			vec![String::from("foo"), String::from("bar")]
		);
	}
}
