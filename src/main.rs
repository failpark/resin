mod cli;
mod conf;
mod git;
mod inputs;
mod utils;

fn main() {
	ctrlc::set_handler(move || {
		let term = console::Term::stderr();
		let _ = term.show_cursor();
	})
	.expect("Error setting ctrl+c handler");

	let args = cli::setup().get_matches();
	let config = conf::Config::get().expect("Failed to read from configuration file");
	let inputs = inputs::get_inputs(&config);
	if let Ok(inputs) = inputs {
		git::commit_changes(&config, &args, &inputs).expect("Failed to commit changes");
	} else {
		let term = console::Term::stderr();
		let _ = term.show_cursor();
		std::process::exit(1);
		// inputs.expect("Failed to get scope");
	}
}
