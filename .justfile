_default:
	@just --list

alias c := clippy

rec:
	asciinema rec -i 1 --cols 85 --rows 20 demo.cast
	agg demo.cast demo.gif

fmt:
	cargo +nightly fmt

clippy:
	cargo clippy