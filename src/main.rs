use std::collections::VecDeque;
use std::env;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::process::exit;
use tiny_http::{Request, Response, Server};

fn get_handler(_: &Request) {
	println!("GET REQUEST");
}

fn post_handler(_: &Request) {
	println!("POST REQUEST");
}

fn start_server(address: SocketAddrV4) {
	let server = Server::http(address);

	match server {
		Ok(server) => {
			for req in server.incoming_requests() {
				println!("{}", req.url().to_string());
				match req.method().as_str() {
					"GET" => get_handler(&req),
					"POST" => post_handler(&req),
					_ => println!("Unknown HTTP method"),
				}

				let response = Response::from_string("hello world");
				req.respond(response).unwrap();
			}
		}
		Err(e) => exit_error(&format!("Can't start server: {}", e)),
	}
}

fn exit_error(message: &str) {
	eprintln!("ERROR: {}", message);
	help();
	exit(1);
}

fn main() {
	let mut args: VecDeque<String> = env::args().skip(1).collect();

	if args.len() == 0 {
		exit_error("0 arguments provided");
	}

	if let Some(subcommand) = args.pop_front() {
		match subcommand.to_lowercase().as_str() {
			"serve" => serve(args),
			//"index" => index(args),
			_ => exit_error("Unknown subcommand"),
		}
	}
}

fn serve(mut args: VecDeque<String>) {
	let mut port: u16 = 8000;
	let mut expose: bool = false;

	while !args.is_empty() {
		if let Some(arg) = args.pop_front() {
			match arg.as_str() {
				"-p" | "--port" => {
					if let Some(custom_port) = args.pop_front() {
						match custom_port.parse::<u16>() {
							Ok(custom_port) => port = custom_port,
							Err(_) => exit_error("Port must be a number"),
						}
					} else {
						exit_error("No port number provided");
					}
				}
				"--host" => {
					expose = true;
				}
				_ => exit_error("Unknown argument"),
			}
		}
	}

	let ip_v4 = if expose {
		Ipv4Addr::new(0, 0, 0, 0)
	} else {
		Ipv4Addr::new(127, 0, 0, 1)
	};

	let address = SocketAddrV4::new(ip_v4, port);
	println!("Starting server on {}", address);
	start_server(address)
}

// fn index(mut args: VecDeque<String>) {}

fn help() {
	println!("lser:");
}
