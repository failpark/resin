use clap::{
	builder::{
		styling::AnsiColor,
		Styles,
	},
	crate_authors,
	crate_description,
	crate_name,
	crate_version,
	Arg,
	Command,
};

pub fn setup() -> Command {
	let styles = Styles::styled()
		.header(AnsiColor::Yellow.on_default())
		.usage(AnsiColor::Green.on_default())
		.literal(AnsiColor::Green.on_default())
		.placeholder(AnsiColor::Green.on_default());
	Command::new(crate_name!())
		.version(crate_version!())
		.author(crate_authors!(",\n"))
		.about(crate_description!())
		.styles(styles)
		.help_template(
			"\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}",
		)
		.arg(
			Arg::new("all")
				.help("Run git add . before committing the the changes")
				.short('a')
				.long("all"),
		)
}
