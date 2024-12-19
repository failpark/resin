use strum::{
	Display,
	EnumIter,
	IntoEnumIterator,
};

// https://www.conventionalcommits.org/en/v1.0.0/#summary
// https://github.com/angular/angular/blob/22b96b9/CONTRIBUTING.md#type
#[derive(Display, EnumIter)]
#[allow(non_camel_case_types)]
pub enum Type {
	feat,
	fix,
	docs,
	style,
	refactor,
	chore,
	build,
	ci,
	perf,
	test,
	revert,
}

impl Type {
	pub fn get_vec() -> Vec<String> {
		Self::iter().map(|val| val.to_string()).collect()
	}
}
