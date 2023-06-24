use crate::commons::exit_error;

use std::collections::VecDeque;
use std::net::{Ipv4Addr, SocketAddrV4};
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

pub fn serve(mut args: VecDeque<String>) {
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
