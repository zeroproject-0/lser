use std::collections::VecDeque;
use std::env;

use lser::commands::index::index;
use lser::commands::serve::serve;
use lser::commons::exit_error;

fn main() {
	let mut args: VecDeque<String> = env::args().skip(1).collect();

	if args.len() == 0 {
		exit_error("0 arguments provided");
	}

	if let Some(subcommand) = args.pop_front() {
		match subcommand.to_lowercase().as_str() {
			"serve" => serve(args),
			"index" => index(args),
			_ => exit_error("Unknown subcommand"),
		}
	}
}
