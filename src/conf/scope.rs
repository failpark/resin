use strum::{
	Display,
	EnumIter,
	IntoEnumIterator,
};

#[derive(Display, EnumIter)]
#[allow(non_camel_case_types)]
pub enum Scope {
	theme,
	block,
	style,
	doc,
	release,
	dev,
	api,
	regression
}

impl Scope {
	pub fn get_vec() -> Vec<String> {
		Scope::iter().map(|val| val.to_string()).collect()
	}
}
