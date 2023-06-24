use std::process::exit;

fn help() {
	println!("lser:");
}

pub fn exit_error(message: &str) {
	eprintln!("ERROR: {}", message);
	help();
	exit(1);
}
