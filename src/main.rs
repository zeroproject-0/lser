use tiny_http::{Response, Server};

fn main() {
	let server = Server::http("0.0.0.0:8000").unwrap();

	for req in server.incoming_requests() {
		if req.method().to_string() == "GET" {
			println!("GETTTTT")
		}

		let response = Response::from_string("hello world");
		req.respond(response).unwrap();
	}

	println!("Hello, world!");
}
