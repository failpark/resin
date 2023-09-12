use clap::{Arg, Command, crate_authors, crate_version, crate_description, crate_name};

pub fn setup() -> Command<'static> {
	Command::new(crate_name!())
		.version(crate_version!())
		.author(crate_authors!("\n"))
		.about(crate_description!())
		.arg(
			Arg::new("all")
				.help("Run git add . before committing the the changes")
				.short('a')
				.long("all"),
		)
		.arg(
			Arg::new("push")
				.help("Run git push after committing the changes")
				.short('p')
				.long("push"),
		)
}
